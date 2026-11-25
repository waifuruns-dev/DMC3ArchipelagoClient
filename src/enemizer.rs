//! Enemy randomizer (enemizer).
//!
//! Every enemy a room spawns goes through the game's "spawn enemy" function (dmc3.exe+1A4680),
//! which takes a CreateEnemyActorData (enemy id at +0, variant at +0x28). We hook it, swap the
//! enemy id for a random one from a safe pool, call the game's function, then put the original id
//! back so the room's own spawn data is never changed.
//!
//! Choices are seeded: the apworld sends `enemizer_seed`, and each pick is a hash of that seed plus
//! the mission, room and either the original enemy (one replacement per enemy type per room) or
//! the spawn's number in the room (every spawn picked separately). The same seed always gives the
//! same enemies.
//!
//! An enemy can only spawn if its model files are loaded, and normally only the mission's own
//! enemies are. This first version only picks replacements whose file set the game already has
//! loaded (read from the game's enemy file list), so it never spawns something that isn't loaded.
//!
//! Enemy ids, file sets, the spawn function and the file list all come from DMC3 Crimson
//! (berthrage/DMC3-Crimson, zlib license), including its notes on which enemies break when
//! spawned this way.

use crate::create_hook;
use crate::data::game_structs::{GameData, SessionData};
use crate::mapping::MAPPING;
use crate::utilities::DMC3_ADDRESS;
use minhook::{MH_STATUS, MinHook};
use std::sync::{Mutex, OnceLock};

pub const SPAWN_ENEMY_ADDR: usize = 0x1A4680;
pub static ORIGINAL_SPAWN_ENEMY: OnceLock<unsafe extern "C" fn(this: usize, data: usize) -> usize> =
    OnceLock::new();

/// `enemy_randomizer` option values (match the apworld)
const MODE_OFF: u8 = 0;
const MODE_PER_TYPE: u8 = 1;
const MODE_MIXED: u8 = 2;

/// Pointer to the game's list of loaded enemy file sets: [[dmc3+C90E10] + 0xA80 + 0x628]
const FILE_POOL_PTR: usize = 0xC90E10;
const FILE_LIST_OFFSET: usize = 0xA80 + 0x628;

/// Enemy ids (Crimson's ENEMY enum) that can be swapped in and out, with the file sets they use.
/// The Seven Hells all share em000, which the game may list under several file set numbers.
const HELL_FILE_SETS: &[u32] = &[0, 4, 10, 15, 28, 29];
struct PoolEnemy {
    id: u32,
    name: &'static str,
    file_sets: &'static [u32],
}
const POOL: &[PoolEnemy] = &[
    PoolEnemy { id: 0, name: "Hell Pride 1", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 1, name: "Hell Pride 2", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 2, name: "Hell Pride 3", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 3, name: "Hell Pride 4", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 4, name: "Hell Gluttony 1", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 5, name: "Hell Gluttony 2", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 6, name: "Hell Gluttony 3", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 7, name: "Hell Gluttony 4", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 8, name: "Hell Lust 1", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 9, name: "Hell Lust 2", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 10, name: "Hell Lust 3", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 11, name: "Hell Lust 4", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 12, name: "Hell Sloth 1", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 13, name: "Hell Sloth 2", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 14, name: "Hell Sloth 3", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 15, name: "Hell Sloth 4", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 16, name: "Hell Wrath 1", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 17, name: "Hell Wrath 2", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 18, name: "Hell Wrath 3", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 19, name: "Hell Wrath 4", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 20, name: "Hell Greed 1", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 21, name: "Hell Greed 2", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 22, name: "Hell Greed 3", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 23, name: "Hell Greed 4", file_sets: HELL_FILE_SETS },
    PoolEnemy { id: 24, name: "Abyss", file_sets: &[1] },
    PoolEnemy { id: 25, name: "Envy", file_sets: &[2] },
    PoolEnemy { id: 28, name: "Arachne", file_sets: &[5] },
    PoolEnemy { id: 29, name: "The Fallen", file_sets: &[6] },
];
// Never swapped either way (bosses, puzzle enemies, and the ones Crimson found buggy or crashy):
// Hell Vanguard, Dullahan, Enigma, Blood-Goyle, Soul Eater, Damned Chessmen, Gigapede, Cerberus,
// Agni & Rudra, Nevan, Geryon, Beowulf, Doppelganger, Arkham, Lady, Vergil, Leviathan, Jester.

fn pool_enemy(id: u32) -> Option<&'static PoolEnemy> {
    POOL.iter().find(|e| e.id == id)
}

/// Spawn numbering for the current room (resets when the mission or room changes)
struct RoomState {
    mission: u32,
    room: i32,
    spawns: u32,
}
static ROOM_STATE: Mutex<RoomState> = Mutex::new(RoomState { mission: 0, room: -1, spawns: 0 });

pub(crate) unsafe fn create_hooks() -> Result<(), MH_STATUS> {
    unsafe {
        create_hook!(
            SPAWN_ENEMY_ADDR,
            spawn_enemy_hook,
            ORIGINAL_SPAWN_ENEMY,
            "Spawn enemy"
        );
    }
    Ok(())
}

extern "C" fn spawn_enemy_hook(this: usize, data: usize) -> usize {
    let Some(orig) = ORIGINAL_SPAWN_ENEMY.get() else {
        return 0;
    };
    if data == 0 {
        return unsafe { orig(this, data) };
    }
    let enemy_ptr = data as *mut u32;
    let enemy = unsafe { enemy_ptr.read_unaligned() };
    let variant = unsafe { ((data + 0x28) as *const u32).read_unaligned() };
    let spawn_method = unsafe { ((data + 0x22) as *const u16).read_unaligned() };

    let (mission, room) = SessionData::with_read(|s| (s.mission, s.room)).unwrap_or((0, -1));
    let spawn_number = {
        let mut state = ROOM_STATE.lock().unwrap_or_else(|e| e.into_inner());
        if state.mission != mission || state.room != room {
            *state = RoomState { mission, room, spawns: 0 };
            log::debug!(
                "Enemizer: room M{} R{}, loaded enemy file sets {:?}",
                mission,
                room,
                loaded_file_sets()
            );
        }
        state.spawns += 1;
        state.spawns
    };

    // Research logging: every spawn, so we can map each room's enemies
    log::debug!(
        "Enemy spawn: M{} R{} #{} enemy {} ({}) variant {} method {}",
        mission,
        room,
        spawn_number,
        enemy,
        pool_enemy(enemy).map_or("not randomized", |e| e.name),
        variant,
        spawn_method
    );

    let Some(new_enemy) = pick_replacement(enemy, mission, room, spawn_number) else {
        return unsafe { orig(this, data) };
    };
    log::debug!(
        "Enemizer: M{} R{} #{} {} -> {}",
        mission,
        room,
        spawn_number,
        pool_enemy(enemy).map_or("?", |e| e.name),
        pool_enemy(new_enemy).map_or("?", |e| e.name)
    );
    unsafe {
        enemy_ptr.write_unaligned(new_enemy);
        let res = orig(this, data);
        // Put the room's own spawn data back the way it was
        enemy_ptr.write_unaligned(enemy);
        if res == 0 {
            log::warn!("Enemizer: the game didn't spawn {}", new_enemy);
        }
        res
    }
}

/// Picks the replacement for a spawn, or None to leave it as it is
fn pick_replacement(enemy: u32, mission: u32, room: i32, spawn_number: u32) -> Option<u32> {
    let (mode, seed) = {
        let mapping = MAPPING.read().ok()?;
        let mapping = mapping.as_ref()?;
        (mapping.enemy_randomizer, mapping.enemizer_seed)
    };
    if mode == MODE_OFF || mission == 0 {
        return None;
    }
    pool_enemy(enemy)?;

    let loaded = loaded_file_sets()?;
    let choices: Vec<u32> = POOL
        .iter()
        .filter(|e| e.file_sets.iter().any(|set| loaded.contains(set)))
        .map(|e| e.id)
        .collect();
    if choices.len() < 2 {
        return None;
    }

    let key = match mode {
        MODE_PER_TYPE => enemy as u64,
        MODE_MIXED => 0x1_0000_0000 | spawn_number as u64,
        _ => return None,
    };
    let hash = mix(seed ^ mix((mission as u64) << 48 ^ (room as u32 as u64) << 32 ^ key));
    Some(choices[(hash % choices.len() as u64) as usize])
}

/// splitmix64 finalizer
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// File set numbers in the game's loaded enemy file list (None if it can't be read)
fn loaded_file_sets() -> Option<Vec<u32>> {
    let pool = read_usize(*DMC3_ADDRESS + FILE_POOL_PTR)?;
    if pool == 0 {
        return None;
    }
    let mut node = read_usize(pool + FILE_LIST_OFFSET)?;
    let mut sets = Vec::new();
    // FileDataMetadata: next at +0x10, category at +0x24, file set at +0x28
    for _ in 0..64 {
        if node == 0 {
            break;
        }
        let set = read_u32(node + 0x28)?;
        if !sets.contains(&set) {
            sets.push(set);
        }
        node = read_usize(node + 0x10)?;
    }
    Some(sets)
}

fn read_usize(addr: usize) -> Option<usize> {
    let mut buf = [0u8; 8];
    crate::skill_manager::safe_read(addr, &mut buf).then(|| usize::from_le_bytes(buf))
}

fn read_u32(addr: usize) -> Option<u32> {
    let mut buf = [0u8; 4];
    crate::skill_manager::safe_read(addr, &mut buf).then(|| u32::from_le_bytes(buf))
}

/// For the hook list (enable/disable with the other hooks)
pub(crate) const HOOK_ADDRESSES: [usize; 1] = [SPAWN_ENEMY_ADDR];
