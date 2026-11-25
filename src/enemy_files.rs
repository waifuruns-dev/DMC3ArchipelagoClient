//! Loads every enemy's model files once, so any enemy can spawn in any room.
//!
//! Normally the game only has the current mission's enemy files loaded, and spawning anything else
//! crashes. This is a port of how DMC3 Crimson (berthrage/DMC3-Crimson, zlib license) solves it:
//!
//! 1. Read all 24 enemy packs (em000.pac ... em037.pac). Crimson's extracted copies in
//!    `data\dmc3\GData.afs\` are used if they exist, otherwise they're read straight out of the
//!    game's archive `data\dmc3\dmc3-0.nbz` (a zip). This part runs on a background thread.
//! 2. On the game thread: copy them into memory below 2 GB (like Crimson's LowAlloc) and let the
//!    game fix up the pointers inside each pack (dmc3.exe+1B9FA0).
//! 3. Build an enemy file list in the game's own format, one entry per enemy id.
//! 4. While a randomized enemy spawns, the game's enemy file list pointer is swapped for ours, so
//!    the game finds that enemy's files whatever the room loaded (see `with_all_enemy_files`).

use crate::utilities::DMC3_ADDRESS;
use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

/// The game's enemy file list pointer: [[dmc3+C90E10] + 0xA80 + 0x628]
const FILE_POOL_PTR: usize = 0xC90E10;
const FILE_LIST_OFFSET: usize = 0xA80 + 0x628;
/// Game function that turns a pack's offsets into pointers
const ADJUST_POINTERS_ADDR: usize = 0x1B9FA0;
/// vtable the game uses for file list entries
const FILE_LIST_FUNCS: usize = 0x4E3218;
const CATEGORY_ENEMY: u32 = 5;
const STATUS_IN_USE: u32 = 3;

/// The 24 enemy packs, in Crimson's ENEMY_FILE_DATA order
const PACKS: [&str; 24] = [
    "em000.pac", "em006.pac", "em007.pac", "em008.pac", "em010.pac", "em011.pac", "em012.pac",
    "em013.pac", "em014.pac", "em016.pac", "em017.pac", "em021.pac", "em023.pac", "em025.pac",
    "em026.pac", "em027.pac", "em028.pac", "em029.pac", "em030.pac", "em031.pac", "em032.pac",
    "em034.pac", "em035.pac", "em037.pac",
];
/// Type names the game expects for each pack (NUL terminated)
const TYPE_NAMES: [&[u8]; 24] = [
    b"obj\\em000.pac\0", b"obj\\em006.pac\0", b"obj\\em007.pac\0", b"obj\\em008.pac\0",
    b"obj\\em010.pac\0", b"obj\\em011.pac\0", b"obj\\em012.pac\0", b"obj\\em013.pac\0",
    b"obj\\em014.pac\0", b"obj\\em016.pac\0", b"obj\\em017.pac\0", b"obj\\em021.pac\0",
    b"obj\\em023.pac\0", b"obj\\em025.pac\0", b"obj\\em026.pac\0", b"obj\\em027.pac\0",
    b"obj\\em028.pac\0", b"obj\\em029.pac\0", b"obj\\em030.pac\0", b"obj\\em031.pac\0",
    b"obj\\em032.pac\0", b"obj\\em034.pac\0", b"obj\\em035.pac\0", b"obj\\em037.pac\0",
];

/// For each of the game's 64 enemy ids: (pack index in PACKS, game file set number).
/// Straight from Crimson's File_Init table; unknown ids point at em000 like Crimson does.
const ENEMY_FILES: [(usize, u32); 64] = {
    let mut t = [(0usize, 0u32); 64];
    // 0-23: the Seven Hells, all em000
    t[24] = (1, 1); // Abyss
    t[25] = (2, 2); // Envy
    t[26] = (3, 3); // Hell Vanguard
    t[28] = (4, 5); // Arachne
    t[29] = (5, 6); // The Fallen
    t[30] = (6, 7); // Dullahan
    t[31] = (7, 8); // Enigma
    t[32] = (8, 9); // Blood-Goyle
    t[34] = (9, 11); // Soul Eater
    t[35] = (10, 12); // Damned Chessmen Pawn
    t[36] = (10, 12); // Knight
    t[37] = (10, 12); // Bishop
    t[38] = (10, 12); // Rook
    t[39] = (11, 13); // Queen
    t[40] = (11, 13); // King
    t[41] = (12, 14); // Gigapede
    t[43] = (13, 16); // Cerberus
    t[45] = (14, 18); // Agni & Rudra
    t[46] = (14, 18); // Agni & Rudra Black
    t[47] = (14, 18); // Agni & Rudra Red
    t[48] = (14, 18); // Agni & Rudra Blue
    t[49] = (16, 19); // Nevan
    t[50] = (17, 20); // Geryon
    t[51] = (18, 21); // Beowulf
    t[52] = (19, 22); // Doppelganger
    t[53] = (20, 23); // Arkham
    t[55] = (21, 25); // Lady
    t[58] = (22, 26); // Vergil
    t[62] = (23, 27); // Jester
    t
};

// Sizes of the game structures (from Crimson's Vars.hpp)
const FILE_DATA_SIZE: usize = 72; // FileData: status +4, typeDataAddr +0x18, file +0x20
const TYPE_DATA_SIZE: usize = 16; // FileDataTypeData: typeName +8
const METADATA_SIZE: usize = 48; // FileDataMetadata: funcs +0, last +8, next +0x10, fileData +0x18, category +0x24, fileSet +0x28

static LOAD_STARTED: AtomicBool = AtomicBool::new(false);
/// Packs read by the background thread, waiting to be set up on the game thread
static READ_PACKS: Mutex<Option<Vec<Vec<u8>>>> = Mutex::new(None);
static LOAD_FAILED: AtomicBool = AtomicBool::new(false);
/// Address of our enemy file list (first entry), once set up
static OUR_LIST: AtomicUsize = AtomicUsize::new(0);
static SETUP_DONE: OnceLock<()> = OnceLock::new();

/// Starts reading the enemy packs in the background (only the first call does anything)
pub(crate) fn start_loading() {
    if LOAD_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(|| {
        let started = std::time::Instant::now();
        let mut packs = Vec::with_capacity(PACKS.len());
        for name in PACKS {
            match read_pack(name) {
                Ok(bytes) => packs.push(bytes),
                Err(err) => {
                    log::error!("Enemizer: couldn't read {}: {}", name, err);
                    LOAD_FAILED.store(true, Ordering::SeqCst);
                    return;
                }
            }
        }
        log::info!(
            "Enemizer: read {} enemy packs ({} MB) in {:.1}s",
            packs.len(),
            packs.iter().map(|p| p.len()).sum::<usize>() / (1024 * 1024),
            started.elapsed().as_secs_f32()
        );
        *READ_PACKS.lock().unwrap_or_else(|e| e.into_inner()) = Some(packs);
    });
}

/// True once every enemy's files are set up. Call from the game thread: the first call after the
/// packs are read does the setup (copy and pointer fix-up).
pub(crate) fn ready() -> bool {
    if OUR_LIST.load(Ordering::SeqCst) != 0 {
        return true;
    }
    if LOAD_FAILED.load(Ordering::SeqCst) || SETUP_DONE.get().is_some() {
        return false;
    }
    let Some(packs) = READ_PACKS.lock().unwrap_or_else(|e| e.into_inner()).take() else {
        return false;
    };
    let _ = SETUP_DONE.set(());
    match unsafe { set_up(&packs) } {
        Some(list) => {
            OUR_LIST.store(list, Ordering::SeqCst);
            log::info!("Enemizer: all enemy files loaded, list at {:#X}", list);
            true
        }
        None => {
            LOAD_FAILED.store(true, Ordering::SeqCst);
            false
        }
    }
}

/// Runs `spawn` with the game's enemy file list pointing at ours, then puts the game's back
pub(crate) fn with_all_enemy_files<R>(spawn: impl FnOnce() -> R) -> R {
    let list = OUR_LIST.load(Ordering::SeqCst);
    let slot = file_list_slot();
    match (list, slot) {
        (0, _) | (_, None) => spawn(),
        (list, Some(slot)) => unsafe {
            let slot = slot as *mut usize;
            let original = slot.read_unaligned();
            slot.write_unaligned(list);
            let res = spawn();
            slot.write_unaligned(original);
            res
        },
    }
}

/// Address of the game's enemy file list pointer, if the game has set it up
fn file_list_slot() -> Option<usize> {
    let pool = read_usize(*DMC3_ADDRESS + FILE_POOL_PTR)?;
    (pool != 0).then_some(pool + FILE_LIST_OFFSET)
}

/// Game enemy file sets currently loaded (enemy entries only), for when ours aren't ready yet
pub(crate) fn loaded_file_sets() -> Option<Vec<u32>> {
    let mut node = read_usize(file_list_slot()?)?;
    let mut sets = Vec::new();
    for _ in 0..128 {
        if node == 0 {
            break;
        }
        if read_u32(node + 0x24)? == CATEGORY_ENEMY {
            let set = read_u32(node + 0x28)?;
            if !sets.contains(&set) {
                sets.push(set);
            }
        }
        node = read_usize(node + 0x10)?;
    }
    Some(sets)
}

unsafe fn set_up(packs: &[Vec<u8>]) -> Option<usize> {
    // Layout: packs (each 0x800 aligned), then FileData[24], FileDataTypeData[24], metadata[64]
    let mut offsets = Vec::with_capacity(packs.len());
    let mut size = 0usize;
    for pack in packs {
        offsets.push(size);
        size = align(size + pack.len(), 0x800);
    }
    let file_data_off = size;
    size += FILE_DATA_SIZE * packs.len();
    let type_data_off = align(size, 16);
    size = type_data_off + TYPE_DATA_SIZE * packs.len();
    let metadata_off = align(size, 16);
    size = metadata_off + METADATA_SIZE * ENEMY_FILES.len();

    let base = alloc_low(size)?;
    log::debug!("Enemizer: {} MB for enemy files at {:#X}", size / (1024 * 1024), base);

    let adjust: unsafe extern "C" fn(usize) =
        unsafe { std::mem::transmute(*DMC3_ADDRESS + ADJUST_POINTERS_ADDR) };

    for (i, pack) in packs.iter().enumerate() {
        let addr = base + offsets[i];
        unsafe {
            std::ptr::copy_nonoverlapping(pack.as_ptr(), addr as *mut u8, pack.len());
            // Crimson's AdjustPointers: archives get each inner file fixed up, others as a whole
            if is_archive(pack) {
                let count = u32::from_le_bytes(pack[4..8].try_into().ok()?) as usize;
                for f in 0..count {
                    let at = 8 + f * 4;
                    let off = u32::from_le_bytes(pack.get(at..at + 4)?.try_into().ok()?) as usize;
                    if off != 0 && off < pack.len() {
                        adjust(addr + off);
                    }
                }
            } else {
                adjust(addr);
            }

            // FileDataTypeData { _: 8, typeName }
            let type_data = base + type_data_off + i * TYPE_DATA_SIZE;
            ((type_data + 8) as *mut usize).write_unaligned(TYPE_NAMES[i].as_ptr() as usize);
            // FileData { group, status, _: 16, typeDataAddr, file, _: 32 }
            let file_data = base + file_data_off + i * FILE_DATA_SIZE;
            ((file_data + 4) as *mut u32).write_unaligned(STATUS_IN_USE);
            ((file_data + 0x18) as *mut usize).write_unaligned(type_data);
            ((file_data + 0x20) as *mut usize).write_unaligned(addr);
        }
    }

    // One list entry per enemy id, linked in order
    let funcs = *DMC3_ADDRESS + FILE_LIST_FUNCS;
    let entry = |i: usize| base + metadata_off + i * METADATA_SIZE;
    for (i, &(pack, file_set)) in ENEMY_FILES.iter().enumerate() {
        let e = entry(i);
        let last = if i > 0 { entry(i - 1) } else { 0 };
        let next = if i + 1 < ENEMY_FILES.len() { entry(i + 1) } else { 0 };
        unsafe {
            (e as *mut usize).write_unaligned(funcs);
            ((e + 8) as *mut usize).write_unaligned(last);
            ((e + 0x10) as *mut usize).write_unaligned(next);
            ((e + 0x18) as *mut usize).write_unaligned(base + file_data_off + pack * FILE_DATA_SIZE);
            ((e + 0x24) as *mut u32).write_unaligned(CATEGORY_ENEMY);
            ((e + 0x28) as *mut u32).write_unaligned(file_set);
        }
    }
    Some(entry(0))
}

fn is_archive(pack: &[u8]) -> bool {
    pack.starts_with(b"PAC") || pack.starts_with(b"PNST")
}

fn align(value: usize, to: usize) -> usize {
    value.div_ceil(to) * to
}

/// Allocates memory between 64 KB and 2 GB, like Crimson's LowAlloc
fn alloc_low(size: usize) -> Option<usize> {
    use windows::Win32::System::Memory::{
        MEM_COMMIT, MEM_FREE, MEM_RESERVE, MEMORY_BASIC_INFORMATION, PAGE_READWRITE, VirtualAlloc,
        VirtualQuery,
    };
    const GRANULARITY: usize = 0x10000;
    let mut pos = GRANULARITY;
    while pos < 0x7FFF_FFFF {
        let mut info = MEMORY_BASIC_INFORMATION::default();
        let got = unsafe {
            VirtualQuery(
                Some(pos as *const _),
                &mut info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        if got == 0 {
            break;
        }
        let region_end = info.BaseAddress as usize + info.RegionSize;
        if info.State == MEM_FREE && region_end.saturating_sub(pos) >= size {
            let addr = unsafe {
                VirtualAlloc(Some(pos as *const _), size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE)
            };
            if !addr.is_null() {
                return Some(addr as usize);
            }
        }
        pos = align(region_end.max(pos + 1), GRANULARITY);
    }
    log::error!("Enemizer: couldn't get {} MB of memory below 2 GB", size / (1024 * 1024));
    None
}

/// Reads an enemy pack: Crimson's extracted copy if present, otherwise from the game's archive
fn read_pack(name: &str) -> std::io::Result<Vec<u8>> {
    let extracted = format!("data\\dmc3\\GData.afs\\{}", name);
    if let Ok(bytes) = std::fs::read(&extracted) {
        return Ok(bytes);
    }
    read_from_zip("data\\dmc3\\dmc3-0.nbz", &format!("GData.afs/{}", name))
}

fn invalid(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg.to_string())
}

fn u16_at(b: &[u8], at: usize) -> std::io::Result<u16> {
    Ok(u16::from_le_bytes(b.get(at..at + 2).ok_or_else(|| invalid("short"))?.try_into().unwrap()))
}
fn u32_at(b: &[u8], at: usize) -> std::io::Result<u32> {
    Ok(u32::from_le_bytes(b.get(at..at + 4).ok_or_else(|| invalid("short"))?.try_into().unwrap()))
}
fn u64_at(b: &[u8], at: usize) -> std::io::Result<u64> {
    Ok(u64::from_le_bytes(b.get(at..at + 8).ok_or_else(|| invalid("short"))?.try_into().unwrap()))
}

/// Central directory of the game archive, read once and kept
static ZIP_DIRECTORY: Mutex<Option<Vec<u8>>> = Mutex::new(None);

/// Minimal zip reader (stored or deflated entries, zip64 aware) for one file in the archive
fn read_from_zip(path: &str, entry_name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut dir_cache = ZIP_DIRECTORY.lock().unwrap_or_else(|e| e.into_inner());
    if dir_cache.is_none() {
        *dir_cache = Some(read_central_directory(&mut file)?);
    }
    let dir = dir_cache.as_ref().unwrap();

    let mut at = 0;
    while at + 46 <= dir.len() && u32_at(dir, at)? == 0x0201_4b50 {
        let method = u16_at(dir, at + 10)?;
        let mut comp_size = u32_at(dir, at + 20)? as u64;
        let mut size = u32_at(dir, at + 24)? as u64;
        let name_len = u16_at(dir, at + 28)? as usize;
        let extra_len = u16_at(dir, at + 30)? as usize;
        let comment_len = u16_at(dir, at + 32)? as usize;
        let mut local_off = u32_at(dir, at + 42)? as u64;
        let name = dir.get(at + 46..at + 46 + name_len).ok_or_else(|| invalid("bad entry"))?;
        let name = String::from_utf8_lossy(name).replace('\\', "/");

        if name.eq_ignore_ascii_case(entry_name) {
            // zip64 extra field: values present for each field that was 0xFFFFFFFF
            let extra = dir
                .get(at + 46 + name_len..at + 46 + name_len + extra_len)
                .ok_or_else(|| invalid("bad extra"))?;
            let mut e = 0;
            while e + 4 <= extra.len() {
                let id = u16_at(extra, e)?;
                let len = u16_at(extra, e + 2)? as usize;
                if id == 0x0001 {
                    let mut p = e + 4;
                    if size == 0xFFFF_FFFF {
                        size = u64_at(extra, p)?;
                        p += 8;
                    }
                    if comp_size == 0xFFFF_FFFF {
                        comp_size = u64_at(extra, p)?;
                        p += 8;
                    }
                    if local_off == 0xFFFF_FFFF {
                        local_off = u64_at(extra, p)?;
                    }
                }
                e += 4 + len;
            }

            let mut header = [0u8; 30];
            file.seek(SeekFrom::Start(local_off))?;
            file.read_exact(&mut header)?;
            if u32_at(&header, 0)? != 0x0403_4b50 {
                return Err(invalid("bad local header"));
            }
            let skip = 30 + u16_at(&header, 26)? as u64 + u16_at(&header, 28)? as u64;
            file.seek(SeekFrom::Start(local_off + skip))?;
            let mut compressed = vec![0u8; comp_size as usize];
            file.read_exact(&mut compressed)?;
            return match method {
                0 => Ok(compressed),
                8 => {
                    let mut out = Vec::with_capacity(size as usize);
                    flate2::read::DeflateDecoder::new(&compressed[..]).read_to_end(&mut out)?;
                    Ok(out)
                }
                _ => Err(invalid(&format!("unsupported compression {}", method))),
            };
        }
        at += 46 + name_len + extra_len + comment_len;
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("{} not in {}", entry_name, path),
    ))
}

fn read_central_directory(file: &mut std::fs::File) -> std::io::Result<Vec<u8>> {
    let len = file.seek(SeekFrom::End(0))?;
    let tail_len = len.min(0x10000 + 22);
    let mut tail = vec![0u8; tail_len as usize];
    file.seek(SeekFrom::Start(len - tail_len))?;
    file.read_exact(&mut tail)?;
    let eocd = (0..=tail.len().saturating_sub(22))
        .rev()
        .find(|&i| tail[i..i + 4] == [0x50, 0x4b, 0x05, 0x06])
        .ok_or_else(|| invalid("no end of central directory"))?;
    let mut dir_size = u32_at(&tail, eocd + 12)? as u64;
    let mut dir_off = u32_at(&tail, eocd + 16)? as u64;
    // zip64: locator sits right before the end record
    if eocd >= 20 && tail[eocd - 20..eocd - 16] == [0x50, 0x4b, 0x06, 0x07] {
        let rec_off = u64_at(&tail, eocd - 20 + 8)?;
        let mut rec = [0u8; 56];
        file.seek(SeekFrom::Start(rec_off))?;
        file.read_exact(&mut rec)?;
        if u32_at(&rec, 0)? == 0x0606_4b50 {
            dir_size = u64_at(&rec, 40)?;
            dir_off = u64_at(&rec, 48)?;
        }
    }
    let mut dir = vec![0u8; dir_size as usize];
    file.seek(SeekFrom::Start(dir_off))?;
    file.read_exact(&mut dir)?;
    Ok(dir)
}

fn read_usize(addr: usize) -> Option<usize> {
    let mut buf = [0u8; 8];
    crate::skill_manager::safe_read(addr, &mut buf).then(|| usize::from_le_bytes(buf))
}

fn read_u32(addr: usize) -> Option<u32> {
    let mut buf = [0u8; 4];
    crate::skill_manager::safe_read(addr, &mut buf).then(|| u32::from_le_bytes(buf))
}
