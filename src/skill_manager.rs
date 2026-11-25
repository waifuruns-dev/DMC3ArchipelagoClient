use crate::game_manager::ArchipelagoData;
use std::collections::HashMap;
use std::ops::BitOrAssign;

use crate::data::game_structs::{CharacterData, GameData, SessionData};
use bitflags::bitflags;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, RwLock};

struct SkillData {
    id: usize,
    index: usize,
    flag: u32,
}

pub static ID_SKILL_MAP: LazyLock<HashMap<usize, &'static str>> = LazyLock::new(|| {
    let mut map: HashMap<usize, &'static str> = SKILLS_MAP
        .iter()
        .map(|(name, data)| (data.id, *name))
        .collect();

    map.extend(HashMap::from([
        (0x53, "Ebony & Ivory Progressive Upgrade"),
        (0x54, "Shotgun Progressive Upgrade"),
        (0x55, "Artemis Progressive Upgrade"),
        (0x56, "Spiral Progressive Upgrade"),
        (0x57, "Kalina Ann Progressive Upgrade"),
    ]));
    map.extend(HashMap::from([
        (0x60, "Progressive Trickster"),
        (0x61, "Progressive Swordmaster"),
        (0x62, "Progressive Gunslinger"),
        (0x63, "Progressive Royalguard"),
    ]));
    map
});

static SKILLS_MAP: LazyLock<HashMap<&str, SkillData>> = LazyLock::new(|| {
    HashMap::from([
        (
            "Rebellion - Stinger Level 1",
            SkillData {
                id: 0x40,
                index: 0,
                flag: Exp0::Rebellion_Stinger_1.bits(),
            },
        ),
        (
            "Rebellion - Stinger Level 2",
            SkillData {
                id: 0x41,
                index: 0,
                flag: 0x100,
            },
        ),
        (
            "Rebellion - Drive",
            SkillData {
                id: 0x42,
                index: 0,
                flag: 0x2000,
            },
        ),
        (
            "Rebellion - Air Hike",
            SkillData {
                id: 0x43,
                index: 6,
                flag: 0x40000,
            },
        ),
        (
            "Cerberus - Revolver Level 2",
            SkillData {
                id: 0x44,
                index: 1,
                // Buying it in the shop sets 0x80 (0x40 is on by default)
                flag: 0x80,
            },
        ),
        (
            "Cerberus - Windmill",
            SkillData {
                id: 0x45,
                index: 1,
                flag: 0x20,
            },
        ),
        (
            "Agni and Rudra - Jet Stream Level 2",
            SkillData {
                id: 0x46,
                index: 1,
                // Buying Jet Stream in the shop first sets 0x8000000 (0x4000000 is on by default)
                flag: 0x8000000,
            },
        ),
        (
            "Agni and Rudra - Jet Stream Level 3",
            SkillData {
                id: 0x47,
                index: 1,
                // The next free Jet Stream flag (confirmed in game)
                flag: 0x10000000,
            },
        ),
        (
            "Agni and Rudra - Whirlwind",
            SkillData {
                id: 0x48,
                index: 1,
                flag: 0x40000000,
            },
        ),
        (
            "Agni and Rudra - Air Hike",
            SkillData {
                id: 0x49,
                index: 6,
                flag: 0x80000,
            },
        ),
        (
            "Nevan - Reverb Shock",
            SkillData {
                id: 0x4A,
                index: 2,
                flag: 0x400000,
            },
        ),
        (
            "Nevan - Reverb Shock Level 2",
            SkillData {
                id: 0x4B,
                index: 2,
                flag: 0x800000,
            },
        ),
        (
            "Nevan - Bat Rift Level 2",
            SkillData {
                id: 0x4C,
                index: 2,
                flag: 0x200000,
            },
        ),
        (
            "Nevan - Air Raid",
            SkillData {
                id: 0x4D,
                index: 3,
                flag: 4,
            },
        ),
        (
            "Nevan - Volume Up",
            SkillData {
                id: 0x4E,
                index: 3,
                flag: 2,
            },
        ),
        (
            "Beowulf - Straight Level 2",
            SkillData {
                id: 0x4F,
                index: 3,
                // Buying it in the shop sets 0x4000000 (0x2000000 is on by default, so it did nothing)
                flag: 0x4000000,
            },
        ),
        (
            "Beowulf - Beast Uppercut",
            SkillData {
                id: 0x50,
                index: 3,
                flag: 0x200000,
            },
        ),
        (
            "Beowulf - Rising Dragon",
            SkillData {
                id: 0x51,
                index: 3,
                flag: 0x400000,
            },
        ),
        (
            "Beowulf - Air Hike",
            SkillData {
                id: 0x52,
                index: 6,
                flag: 0x100000,
            },
        ),
    ])
});
static DEFAULT_SKILLS: [u32; 8] = [
    // I should see what else this lets me control...
    0xFFFF5E7F, 0xA7FFAF5F, 0xAF1FFFF3, 0xCB9FFFF9, 0xFBFBFFFE, 0xFFFFEFFD, 0xFFE3FEFF, 0xFFFFFFFF,
];

/*
   Notes:
       - Free Ride, Taunt, Jump/Wall Jump are all built in
       TODO Skill notes
       - Need to check Vergil stuff
       - Need to run through all skills again, see if there's any holes
       - To Test/Find
           - Rebellion - High Time, Float (Glide), Slash Roll (Pole Play with Rebellion)
           - A&R - Million Slash (Crazy Combo 3)
           - Nevan - Thunder Bolt and Vortex (Air raid abilities)
           - Beowulf - Tornado (Crazy RI)
           - E&I - Wild Stomp, Rain Storm
           - Shotgun - Point Blank, finisher for gun stinger
           - Artemis - Acid Rain
           - Spiral - Reflector
           - Kalina Ann - More Grapple testing, might be multistep
*/
bitflags! {
    pub struct Exp0: u32 {
        // Default                        0b1111_1111_1111_1111_0101_1110_0111_1111;
        const Unknown =                   0b0011_1110_1100_0010_0001_1000_0000_0001;
        // Combo 1 - Need all three in order to do combo 1
        const Rebellion_Combo_A_1 =       0b0000_0000_0000_0000_0000_0000_0000_0010;
        const Rebellion_Combo_A_2 =       0b0000_0000_0000_0000_0000_0000_0000_0100;
        const Rebellion_Combo_A_3 =       0b0000_0000_0000_0000_0000_0000_0000_1000;
        // Combo 2 - Need Combo A-1
        const Rebellion_Combo_B_1 =       0b0000_0000_0000_0000_0000_0000_0001_0000;
        const Yamato_Rapid_Slash_1 =      0b0000_0000_0000_0000_0000_0000_0001_0000;
        const Rebellion_Combo_B_2 =       0b0000_0000_0000_0000_0000_0000_0010_0000;
        const Yamato_Rapid_Slash_2 =      0b0000_0000_0000_0000_0000_0000_0010_0000;
        const Rebellion_Helm_Breaker =    0b0000_0000_0000_0000_0000_0000_0100_0000;
        // Store
        const Rebellion_Stinger_1 =       0b0000_0000_0000_0000_0000_0000_1000_0000;
        const Rebellion_Stinger_2 =       0b0000_0000_0000_0000_0000_0001_0000_0000;
        // Not confirmed
        const Yamato_Judgement_Cut_1 =    0b0000_0000_0000_0000_0000_0010_0000_0000;
        const Yamato_Judgement_Cut_2 =    0b0000_0000_0000_0000_0000_0100_0000_0000;
        const Rebellion_Drive =           0b0000_0000_0000_0000_0010_0000_0000_0000;
        // Needs Combo 2
        const Rebellion_Million_Stabs =   0b0000_0000_0000_0000_0100_0000_0000_0000;
        const Rebellion_Sword_Pierce =    0b0000_0000_0000_0000_1000_0000_0000_0000;
        // First half
        const Rebellion_Prop_Shredder_1 = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        // Needs Sword Pierce
        const Unarmed_Kick =              0b0000_0000_0000_0100_0000_0000_0000_0000;
        // Aerial Wave
        const Rebellion_Aerial_Wave_1 =   0b0000_0000_0000_1000_0000_0000_0000_0000;
        const Rebellion_Aerial_Wave_2 =   0b0000_0000_0001_0000_0000_0000_0000_0000;
        const Rebellion_Aerial_Wave_3 =   0b0000_0000_0010_0000_0000_0000_0000_0000;
        // Needs prop shredder 1
        const Rebellion_Prop_Shredder_2 = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        // Cerberus Combo 1
        const Cerberus_Combo_A_1 =        0b0100_0000_0000_0000_0000_0000_0000_0000;
        const Cerberus_Combo_A_2 =        0b1000_0000_0000_0000_0000_0000_0000_0000;

        const Cerberus_Combo_A_B =        0b1100_0000_0000_0000_0000_0000_0000_0000;
    }

    pub struct Exp1: u32 {
        // Default                        0b1010_0111_1111_1111_1010_1111_0101_1111;
        const Unknown =                   0b1000_0000_0000_0011_1000_0000_0000_0000;
        // Continued Cerberus Combo 1
        const Cerberus_Combo_A_3 =        0b0000_0000_0000_0000_0000_0000_0000_0001;
        const Cerberus_Combo_A_4 =        0b0000_0000_0000_0000_0000_0000_0000_0010;
        const Cerberus_Combo_A_5 =        0b0000_0000_0000_0000_0000_0000_0000_0100;
        const Force_Edge_Helm_Breaker =   0b0000_0000_0000_0000_0000_0000_0000_0100;
        // Cerberus Combo 2 - Needs A1 and A2
        const Cerberus_Combo_B_1 =        0b0000_0000_0000_0000_0000_0000_0000_1000;
        const Cerberus_Combo_B_2 =        0b0000_0000_0000_0000_0000_0000_0001_0000;
        const Cerberus_Windmill =         0b0000_0000_0000_0000_0000_0000_0010_0000;
        const Cerberus_Revolver_2 =       0b0000_0000_0000_0000_0000_0000_0100_0000;
        // Also Revolver Lv2
        const Force_Edge_Stinger_1 =      0b0000_0000_0000_0000_0000_0000_0100_0000;
        // Also Revolver
        const Force_Edge_Stinger_2 =      0b0000_0000_0000_0000_0000_0000_1000_0000;
        const Cerberus_Revolver =         0b0000_0000_0000_0000_0000_0000_1000_0000;
        const Force_Edge_Round_Trip =     0b0000_0000_0000_0000_0000_0001_0000_0000;
        const Cerberus_Swing =            0b0000_0000_0000_0000_0000_0001_0000_0000;
        // Combo 2 Crazy
        const Cerberus_Satellite =        0b0000_0000_0000_0000_0000_0010_0000_0000;
        // Style
        const Cerberus_Flicker =          0b0000_0000_0000_0000_0000_0100_0000_0000;
        const Cerberus_Air_Flicker =      0b0000_0000_0000_0000_0000_1000_0000_0000;
        const Cerberus_Crystal =          0b0000_0000_0000_0000_0001_0000_0000_0000;
        const Cerberus_Million_Carats =   0b0000_0000_0000_0000_0010_0000_0000_0000;
        const Cerberus_Ice_Age =          0b0000_0000_0000_0000_0100_0000_0000_0000;
        // Agni & Rudra Combo 1
        const AgniAndRudra_Combo_A_1 =    0b0000_0000_0000_0100_0000_0000_0000_0000;
        const AgniAndRudra_Combo_A_2 =    0b0000_0000_0000_1000_0000_0000_0000_0000;
        const AgniAndRudra_Combo_A_3 =    0b0000_0000_0001_0000_0000_0000_0000_0000;
        const AgniAndRudra_Combo_A_4 =    0b0000_0000_0010_0000_0000_0000_0000_0000;
        const AgniAndRudra_Combo_A_5 =    0b0000_0000_0100_0000_0000_0000_0000_0000;
        // Agni & Rudra Combo 2 - Requires A-1 to start
        const AgniAndRudra_Combo_B_1 =    0b0000_0000_1000_0000_0000_0000_0000_0000;
        const AgniAndRudra_Combo_B_2 =    0b0000_0001_0000_0000_0000_0000_0000_0000;
        // Agni & Rudra Combo 3 - Requires A-1 and B-1
        const AgniAndRudra_Combo_C =      0b0000_0010_0000_0000_0000_0000_0000_0000;
        const AgniAndRudra_JetStream_2 =  0b0000_0100_0000_0000_0000_0000_0000_0000;
        const AgniAndRudra_JetStream_3 =  0b0000_1000_0000_0000_0000_0000_0000_0000;
        const AgniAndRudra_Aerial_Cross = 0b0010_0000_0000_0000_0000_0000_0000_0000;
        const AgniAndRudra_Whirlwind =    0b0100_0000_0000_0000_0000_0000_0000_0000;
    }

    pub struct Exp2: u32 {
        // Default                        0b1010_1111_0001_1111_1111_1111_1111_0011;
        const Unknown =                   0b1000_0000_0000_1100_1111_1111_1000_0001;
        const AgniAndRudra_Cross_Swords = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        // Both Twister+Tempest?
        const AgniAndRudra_Crawler =      0b0000_0000_0000_0000_0000_0000_0000_0100;
        const AgniAndRudra_Tempest =      0b0000_0000_0000_0000_0000_0000_0000_1000;
        // Agni and Rudra - Sky Dance
        // First slashes
        const AgniAndRudra_Sky_Dance_1 =  0b0000_0000_0000_0000_0000_0000_0001_0000;
        // Second slashes, seems to be fine on its own
        const AgniAndRudra_Sky_Dance_2 =  0b0000_0000_0000_0000_0000_0000_0010_0000;
        // Spin down to the ground, independent
        const AgniAndRudra_Sky_Dance_3 =  0b0000_0000_0000_0000_0000_0000_0100_0000;
        const Nevan_Tune_Up =             0b0000_0000_0000_0001_0000_0000_0000_0000;
        // Needs Tune Up, Also includes Jam Session
        const Neva_Combo_ABC =            0b0000_0000_0000_0010_0000_0000_0000_0000;
        const Nevan_Bat_Rift =            0b0000_0000_0001_0000_0000_0000_0000_0000;
        const Nevan_Bat_Rift_2 =          0b0000_0000_0010_0000_0000_0000_0000_0000;
        const Nevan_Reverb_Shock_1 =      0b0000_0000_0100_0000_0000_0000_0000_0000;
        const Nevan_Reverb_Shock_2 =      0b0000_0000_1000_0000_0000_0000_0000_0000;
        const Nevan_Air_Play =            0b0000_0001_0000_0000_0000_0000_0000_0000;
        const Nevan_Slash =               0b0000_0010_0000_0000_0000_0000_0000_0000;
        const Nevan_Air_Slash =           0b0000_0100_0000_0000_0000_0000_0000_0000;
        // Style
        const Nevan_Feedback =            0b0001_0000_0000_0000_0000_0000_0000_0000;
        const Nevan_Crazy_Roll =          0b0010_0000_0000_0000_0000_0000_0000_0000;
        const Nevan_Distortion =          0b0100_0000_0000_0000_0000_0000_0000_0000;
    }

    pub struct Exp3: u32 {
        // Default                        0b1100_1011_1001_1111_1111_1111_1111_1001;
        const Unknown =                   0b0100_0001_1000_0000_0001_1111_1111_1001;
        const Nevan_Volume_Up =           0b0000_0000_0000_0000_0000_0000_0000_0010;
        const Nevan_Air_Raid =            0b0000_0000_0000_0000_0000_0000_0000_0100;
        // Beowulf Combo 1
        const Beowulf_Combo_A_1 =         0b0000_0000_0000_0000_0100_0000_0000_0000;
        const Beowulf_Combo_A_2 =         0b0000_0000_0000_0000_1000_0000_0000_0000;
        const Beowulf_Combo_A_3 =         0b0000_0000_0000_0001_0000_0000_0000_0000;
        // Beowulf Combo 2 - Needs A-1 and A-2
        const Beowulf_Combo_B_1 =         0b0000_0000_0000_0010_0000_0000_0000_0000;
        const Beowulf_Combo_B_2 =         0b0000_0000_0000_0100_0000_0000_0000_0000;
        // Crazy Combo for Beowulf Combo 2, doesn't need B-2
        const Beowulf_Combo_Hyper_Fist =  0b0000_0000_0000_1000_0000_0000_0000_0000;
        const Beowulf_Killer_Bee =        0b0000_0000_0001_0000_0000_0000_0000_0000;
        const Beowulf_Beast_Uppercut =    0b0000_0000_0010_0000_0000_0000_0000_0000;
        const Beowulf_Rising_Dragon =     0b0000_0000_0100_0000_0000_0000_0000_0000;
        // Might be backwards? Both appear to be straight
        const Beowulf_Straight_2 =        0b0000_0010_0000_0000_0000_0000_0000_0000;
        const Beowulf_Straight_Q =        0b0000_0100_0000_0000_0000_0000_0000_0000;
        // Style
        const Beowulf_Zodiac =            0b0000_1000_0000_0000_0000_0000_0000_0000;
        const Beowulf_Ground_Volcano =    0b0001_0000_0000_0000_0000_0000_0000_0000;
        const Beowulf_Air_Volcano =       0b0010_0000_0000_0000_0000_0000_0000_0000;
        const Beowulf_Hammer =            0b1000_0000_0000_0000_0000_0000_0000_0000;
    }

    pub struct Exp4: u32 {
        // Default                        0b1111_1011_1111_1011_1111_1111_1111_1110;
        const Unknown =                   0b1111_1010_0011_1000_1000_1111_1010_0010;
        const Beowulf_Real_Impact =       0b0000_0000_0000_0000_0000_0000_0000_0001;
        const Ebony_And_Ivory_Shoot =     0b0000_0000_0000_0000_0000_0000_0000_0100;
        const EbonyIvory_Charge_Shot =    0b0000_0000_0000_0000_0000_0000_0000_1000;
        const EbonyIvory_Air_Shoot =      0b0000_0000_0000_0000_0000_0000_0001_0000;
        const Gunslinger_Twosome_Time =   0b0000_0000_0000_0000_0000_0000_0100_0000;
        const Shotgun_Shoot =             0b0000_0000_0000_0000_0001_0000_0000_0000;
        const Shotgun_Charge_Shot =       0b0000_0000_0000_0000_0010_0000_0000_0000;
        const Shotgun_Air_Shoot =         0b0000_0000_0000_0000_0100_0000_0000_0000;
        const Shotgun_Fireworks =         0b0000_0000_0000_0001_0000_0000_0000_0000;
        const Shotgun_Fireworks_Air =     0b0000_0000_0000_0010_0000_0000_0000_0000;
        const Shotgun_Stinger =           0b0000_0000_0000_0100_0000_0000_0000_0000;
        const Artemis_Shoot =             0b0000_0000_0100_0000_0000_0000_0000_0000;
        const Artemis_Air_Shoot =         0b0000_0000_1000_0000_0000_0000_0000_0000;
        const Artemis_Multilock =         0b0000_0001_0000_0000_0000_0000_0000_0000;
        const Artemis_Sphere =            0b0000_0100_0000_0000_0000_0000_0000_0000;
    }

    pub struct Exp5: u32 {
         // Default                       0b1111_1111_1111_1111_1110_1111_1111_1101;
        const Unknown =                   0b1001_1111_0000_1111_1110_0011_1111_1000;
        const Spiral_Shoot =              0b0000_0000_0000_0000_0000_0000_0000_0001;
        const Spiral_Sniper =             0b0000_0000_0000_0000_0000_0000_0000_0010;
        const Spiral_Trick_Shot =         0b0000_0000_0000_0000_0000_0000_0000_0100;
        const Kalina_Ann_Shoot =          0b0000_0000_0000_0000_0000_0100_0000_0000;
        const Kalina_Ann_Hysteria =       0b0000_0000_0000_0000_0000_1000_0000_0000;
        const Kalina_Ann_Grapple =        0b0000_0000_0000_0000_0001_0000_0000_0000;
        // Amount of dashes seems to depend on style level, not additional flags
        const Trickster_Dash =            0b0000_0000_0001_0000_0000_0000_0000_0000;
        const Trickster_Sky_Star =        0b0000_0000_0010_0000_0000_0000_0000_0000;
        const Trickster_Air_Trick =       0b0000_0000_0100_0000_0000_0000_0000_0000;
        const Trickster_Wall_Hike =       0b0000_0000_1000_0000_0000_0000_0000_0000;
        const Royalguard_Release =        0b0100_0000_0000_0000_0000_0000_0000_0000;
    }

    pub struct Exp6: u32 {
        // Default                        0b1111_1111_1110_0011_1111_1110_1111_1111;
        const Unknown =                   0b1111_1111_1110_0000_0000_0000_1111_1111;
        const Royalguard_Release_Air =    0b0000_0000_0000_0000_0000_0000_0000_0100;
        // The wall of dance macabre
        const Dance_Macabre_1 =           0b0000_0000_0000_0000_0000_0001_0000_0000;
        const Dance_Macabre_2 =           0b0000_0000_0000_0000_0000_0010_0000_0000;
        const Dance_Macabre_3 =           0b0000_0000_0000_0000_0000_0100_0000_0000;
        const Dance_Macabre_4 =           0b0000_0000_0000_0000_0000_1000_0000_0000;
        const Dance_Macabre_5 =           0b0000_0000_0000_0000_0001_0000_0000_0000;
        const Dance_Macabre_6 =           0b0000_0000_0000_0000_0010_0000_0000_0000;
        const Dance_Macabre_7 =           0b0000_0000_0000_0000_0100_0000_0000_0000;
        const Dance_Macabre_8 =           0b0000_0000_0000_0000_1000_0000_0000_0000;
        const Dance_Macabre_9 =           0b0000_0000_0000_0001_0000_0000_0000_0000;
        //const Dance_Macabre_All =       0b0000_0000_0000_0001_1111_1111_0000_0000;
        const Poll_Play =                 0b0000_0000_0000_0010_0000_0000_0000_0000;
        const Rebellion_Air_Hike =        0b0000_0000_0000_0100_0000_0000_0000_0000;
        const AgniAndRudra_Air_Hike =     0b0000_0000_0000_1000_0000_0000_0000_0000;
        const Beowulf_Air_Hike =          0b0000_0000_0001_0000_0000_0000_0000_0000;
    }

    pub struct Exp7: u32 { // ??
         // Default                       0b1111_1111_1111_1111_1111_1111_1111_1111;
         const Unknown =                  0b0000_0000_0000_0000_0000_0000_0000_0000;
    }
}

pub(crate) fn reset_expertise() {
    SessionData::with_mut(|s| {
        s.expertise = DEFAULT_SKILLS;
    })
    .expect("Unable to reset expertise");
    let _ = CharacterData::with_mut(|c| {
        c.expertise = DEFAULT_SKILLS;
    });
}

fn give_skill(skill_id: &usize) {
    // This works, might not update files? need to double-check
    let data = SKILLS_MAP.get(ID_SKILL_MAP.get(skill_id).unwrap()).unwrap();
    SessionData::with_mut(|s| {
        s.expertise[data.index].bitor_assign(data.flag);
    })
    .expect("Unable to give skill");

    let _ = CharacterData::with_mut(|c| {
        c.expertise[data.index].bitor_assign(data.flag);
    });
}

/// The skill flags Dante should have right now, written by set_skills.
static EXPECTED_EXPERTISE: RwLock<Option<[u32; 8]>> = RwLock::new(None);

/// Called every game tick. The game rebuilds Dante's live skill flags on death and room
/// change from copies the mod doesn't write, which dropped received skills until a mission
/// restart. If the live flags no longer match what set_skills applied, put them back.
/// Only writes Dante's live copy, which set_skills already writes.
pub(crate) fn enforce_skills() {
    // Only when skills are randomized; otherwise skills bought in the shop must stick
    match crate::mapping::MAPPING.try_read() {
        Ok(mapping) if mapping.as_ref().is_some_and(|m| m.randomize_skills) => {}
        _ => return,
    }
    let Ok(expected) = EXPECTED_EXPERTISE.try_read() else {
        return;
    };
    let Some(expected) = *expected else {
        return;
    };
    let _ = CharacterData::with_mut(|c| {
        if c.expertise != expected {
            log::debug!("Game reset Dante's skills, re-applying received skills");
            c.expertise = expected;
        }
    });
}

pub(crate) fn clear_expected_skills() {
    if let Ok(mut expected) = EXPECTED_EXPERTISE.write() {
        *expected = None;
    }
}

pub(crate) fn set_skills(data: &ArchipelagoData) {
    // I kinda don't like this tbh, but oh well, shouldn't really be an issue.
    reset_expertise();
    let mut ids: Vec<_> = data.skills.iter().map(|id| format!("{:#X}", id)).collect();
    ids.sort();
    log::debug!(
        "Applying skills: [{}] (Stinger {}, Jet Stream {}, Reverb {}, Uppercut {})",
        ids.join(", "),
        data.stinger_level,
        data.jet_stream_level,
        data.reverb_level,
        data.beowulf_level
    );
    for skill in data.skills.iter() {
        give_skill(skill);
    }
    let mut expected = DEFAULT_SKILLS;
    for skill in data.skills.iter() {
        let skill_data = SKILLS_MAP.get(ID_SKILL_MAP.get(skill).unwrap()).unwrap();
        expected[skill_data.index] |= skill_data.flag;
    }
    if let Ok(mut stored) = EXPECTED_EXPERTISE.write() {
        *stored = Some(expected);
    }
}

// Certain skills have two levels they can gain
pub(crate) fn add_skill(id: usize, data: &mut ArchipelagoData) {
    match id {
        0x40 => {
            data.add_stinger_level();
        }
        0x46 => {
            data.add_jet_stream_level();
        }
        0x4A => {
            data.add_reverb_level();
        }
        0x50 => {
            data.add_beowulf_level();
        }
        _ => {}
    }

    let skill_id = match id {
        0x40 => match data.stinger_level {
            1 => 0x40,
            2 => 0x41,
            _ => unreachable!(),
        },
        0x46 => match data.jet_stream_level {
            1 => 0x46,
            2 => 0x47,
            _ => unreachable!(),
        },
        0x4A => match data.reverb_level {
            1 => 0x4A,
            2 => 0x4B,
            _ => unreachable!(),
        },
        0x50 => match data.beowulf_level {
            1 => 0x50,
            2 => 0x51,
            _ => unreachable!(),
        },
        _ => id,
    };
    data.add_skill(skill_id);
}

// ---------------------------------------------------------------------------------------------
// Skill shop checks, done the way AshIndigo's gun shop works:
// - opening: back up the shop's own skill list, then show the finished purchase checks in it
// - buying: any skill that appears in the list is a purchase; send its check straight away
// - leaving: put the backed-up list back before the game closes the shop, so the game copies the
//   real skills (not purchases) into Dante's lists
// Dante's own skills always come from received items (enforce_skills).

/// True while the skill shop is open with purchase checks on
pub(crate) static IN_SKILL_SHOP: AtomicBool = AtomicBool::new(false);

/// The open shop: where its lists are, the backed-up real list, and what it shows now
struct OpenShop {
    list: usize,
    result: usize,
    backup: [u32; 8],
    shown: [u32; 8],
}
static OPEN_SHOP: RwLock<Option<OpenShop>> = RwLock::new(None);

/// Purchases already sent this session (so one purchase is never sent twice)
static REPORTED_PURCHASES: LazyLock<RwLock<std::collections::HashSet<&'static str>>> =
    LazyLock::new(|| RwLock::new(std::collections::HashSet::new()));

/// Skill flags for the purchase checks already done
pub(crate) fn purchase_check_expertise<'a>(checked: impl Iterator<Item = &'a str>) -> [u32; 8] {
    let mut flags = DEFAULT_SKILLS;
    for name in checked {
        if let Some(skill) = name.strip_prefix("Purchase ")
            && let Some(data) = SKILLS_MAP.get(skill)
        {
            flags[data.index] |= data.flag;
        }
    }
    flags
}

/// Names of shop skills whose flag is set in `new` but not in `old`
fn newly_bought(old: &[u32; 8], new: &[u32; 8]) -> Vec<&'static str> {
    SKILLS_MAP
        .iter()
        .filter(|(_, d)| new[d.index] & d.flag != 0 && old[d.index] & d.flag == 0)
        .map(|(name, _)| *name)
        .collect()
}

/// Shop opens: back up its list and show the finished purchase checks instead
pub(crate) fn open_skill_shop(checks: [u32; 8]) {
    IN_SKILL_SHOP.store(true, Ordering::SeqCst);
    let opened = find_shop_lists().and_then(|(list, result)| {
        let backup = read_copy(list)?;
        write_list(list, checks);
        Some(OpenShop { list, result, backup, shown: checks })
    });
    match &opened {
        Some(shop) => log::debug!(
            "Skill shop opened: list at {:#X}, backed up {:X?}, showing purchase checks {:X?}",
            shop.list,
            shop.backup,
            checks
        ),
        None => {
            log::warn!("Skill shop: couldn't find the shop's skill list; purchases won't be checks");
            log_skill_lists_everywhere();
        }
    }
    if let Ok(mut open) = OPEN_SHOP.write() {
        *open = opened;
    }
}

/// Skills bought since the last call: new flags in the shop's list
pub(crate) fn take_new_purchases() -> Vec<&'static str> {
    let Ok(mut open) = OPEN_SHOP.write() else {
        return vec![];
    };
    let Some(shop) = open.as_mut() else {
        return vec![];
    };
    let Some(now) = read_copy(shop.list) else {
        return vec![];
    };
    if now == shop.shown {
        return vec![];
    }
    let mut bought = newly_bought(&shop.shown, &now);
    for (index, word) in now.iter().enumerate() {
        let unknown = word & !shop.shown[index] & !purchase_bits(index);
        if unknown != 0 {
            log::warn!("Skill shop: bought a skill with unknown flag {:#X} in word {}", unknown, index);
        }
    }
    shop.shown = now;
    if let Ok(mut reported) = REPORTED_PURCHASES.write() {
        bought.retain(|name| !reported.contains(name));
        reported.extend(bought.iter().copied());
    }
    if !bought.is_empty() {
        log::debug!("Skill shop: bought {:?}", bought);
    }
    bought
}

/// When the shop's list isn't found: log every skill list in game memory once per session, so we
/// can see where this shop keeps it (read-only and crash-safe, but takes a second or two)
fn log_skill_lists_everywhere() {
    use windows::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
    };
    static DONE: AtomicBool = AtomicBool::new(false);
    if DONE.swap(true, Ordering::SeqCst) {
        return;
    }
    let mut ptr = [0u8; 8];
    let object = if safe_read(*crate::DMC3_ADDRESS + SHOP_OBJECT_POINTER, &mut ptr) {
        u64::from_le_bytes(ptr) as usize
    } else {
        0
    };
    let mut hits = Vec::new();
    let mut addr = 0x10000usize;
    let mut buf = Vec::new();
    while addr < 0x7FFF_FFFF_0000 && hits.len() < 40 {
        let mut info = MEMORY_BASIC_INFORMATION::default();
        if unsafe {
            VirtualQuery(
                Some(addr as *const _),
                &mut info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        } == 0
        {
            break;
        }
        let base = info.BaseAddress as usize;
        let size = info.RegionSize;
        if info.State == MEM_COMMIT
            && (info.Protect.0 & (PAGE_NOACCESS.0 | PAGE_GUARD.0)) == 0
            && size <= 0x4000_0000
        {
            buf.resize(size, 0);
            if safe_read(base, &mut buf) {
                let mut off = 0;
                while off + 32 <= size {
                    let mut w = [0u32; 8];
                    for (k, v) in w.iter_mut().enumerate() {
                        *v = u32::from_le_bytes(buf[off + k * 4..off + k * 4 + 4].try_into().unwrap());
                    }
                    if looks_like_skill_list(w) {
                        hits.push(format!("{:#X} {:X?}", base + off, &w[..4]));
                        off += 32;
                    } else {
                        off += 4;
                    }
                }
            }
        }
        addr = base + size.max(0x1000);
    }
    log::debug!(
        "Skill shop object pointer {:#X}; skill lists in memory: {}",
        object,
        hits.join(" | ")
    );
}

/// Every purchase flag in one word of the list (for the unknown flag warning)
fn purchase_bits(index: usize) -> u32 {
    SKILLS_MAP.values().filter(|d| d.index == index).fold(0, |acc, d| acc | d.flag)
}

/// Shop closes: put the backed-up list back (call before the game closes the shop), then keep
/// Dante's received skills
pub(crate) fn close_skill_shop(data: &ArchipelagoData) {
    if let Ok(mut open) = OPEN_SHOP.write()
        && let Some(shop) = open.take()
    {
        write_list(shop.list, shop.backup);
        if read_copy(shop.result).is_some_and(looks_like_skill_list) {
            write_list(shop.result, shop.backup);
        }
        log::debug!("Skill shop closed: list put back to {:X?}", shop.backup);
    }
    IN_SKILL_SHOP.store(false, Ordering::SeqCst);
    set_skills(data);
}

/// Reads game memory without risking a crash: an unreadable or freed address returns None
fn safe_read(addr: usize, buf: &mut [u8]) -> bool {
    use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
    use windows::Win32::System::Threading::GetCurrentProcess;
    let mut read = 0usize;
    unsafe {
        ReadProcessMemory(
            GetCurrentProcess(),
            addr as *const _,
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            Some(&mut read),
        )
        .is_ok()
            && read == buf.len()
    }
}

fn read_copy(addr: usize) -> Option<[u32; 8]> {
    let mut buf = [0u8; 32];
    if !safe_read(addr, &mut buf) {
        return None;
    }
    let mut words = [0u32; 8];
    for (i, w) in words.iter_mut().enumerate() {
        *w = u32::from_le_bytes(buf[i * 4..i * 4 + 4].try_into().unwrap());
    }
    Some(words)
}


// The skill shop's own lists. The game copies the saved skills into the shop's list when the
// Divinity Statue menu opens; the shop shows bought/unbought from it and adds purchases to it. A
// second list sits 0xD0 after it. Both are on an object found through a pointer in the game's
// data, but their position on it moves (+0x9E4 in one session, +0x21E4 in another), so they're
// found by searching the object for two skill lists 0xD0 apart.
const SHOP_OBJECT_POINTER: usize = 0xC9DDB8;
/// The menu shop's list has been at +0x9E4 to +0x41E4, the mid-mission statue's at +0x1CC54
const SHOP_SEARCH_SIZE: usize = 0x40000;
/// Read in pieces, so one unreadable page only ends the search there
const SHOP_SEARCH_CHUNK: usize = 0x10000;
const SHOP_RESULT_DISTANCE: usize = 0xD0;

/// The last four words of a skill list never change (no purchasable skills in them)
fn looks_like_skill_list(w: [u32; 8]) -> bool {
    w[4..] == DEFAULT_SKILLS[4..]
}

/// Addresses of the shop's list and the list 0xD0 after it
fn find_shop_lists() -> Option<(usize, usize)> {
    let mut ptr = [0u8; 8];
    if !safe_read(*crate::DMC3_ADDRESS + SHOP_OBJECT_POINTER, &mut ptr) {
        return None;
    }
    let base = u64::from_le_bytes(ptr) as usize;
    if base == 0 {
        return None;
    }
    let overlap = SHOP_RESULT_DISTANCE + 32;
    let mut buf = vec![0u8; SHOP_SEARCH_CHUNK + overlap];
    let mut start = 0;
    while start < SHOP_SEARCH_SIZE {
        if !safe_read(base + start, &mut buf) {
            return None;
        }
        let words = |at: usize| -> [u32; 8] {
            let mut w = [0u32; 8];
            for (k, v) in w.iter_mut().enumerate() {
                *v = u32::from_le_bytes(buf[at + k * 4..at + k * 4 + 4].try_into().unwrap());
            }
            w
        };
        if let Some(off) = (0..SHOP_SEARCH_CHUNK).step_by(4).find(|&off| {
            looks_like_skill_list(words(off))
                && looks_like_skill_list(words(off + SHOP_RESULT_DISTANCE))
        }) {
            let list = base + start + off;
            return Some((list, list + SHOP_RESULT_DISTANCE));
        }
        start += SHOP_SEARCH_CHUNK;
    }
    None
}

fn write_list(addr: usize, flags: [u32; 8]) {
    unsafe { std::ptr::write_unaligned(addr as *mut [u32; 8], flags) }
}
