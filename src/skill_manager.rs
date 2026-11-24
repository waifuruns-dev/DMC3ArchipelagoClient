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
                flag: 0x40,
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
                flag: 0x4000000,
            },
        ),
        (
            "Agni and Rudra - Jet Stream Level 3",
            SkillData {
                id: 0x47,
                index: 1,
                flag: 0x8000000,
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
// Skill shop checks: while the shop is open, Dante's skill flags show which purchase checks are
// done (not which skills were received), so every unbought check is for sale. Buying one sets
// its flag; the new flag is turned into a "Purchase <skill>" check. Received skills come back
// when the shop closes.

/// True while the skill shop is open with purchase checks on
pub(crate) static IN_SKILL_SHOP: AtomicBool = AtomicBool::new(false);
/// Skill flags at the last check, to spot newly bought skills
static SHOP_SNAPSHOT: RwLock<Option<[u32; 8]>> = RwLock::new(None);

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

/// Shows purchase checks in the shop. Call before the shop builds its menu.
pub(crate) fn open_skill_shop(flags: [u32; 8]) {
    IN_SKILL_SHOP.store(true, Ordering::SeqCst);
    // Nothing is written to the game here: the shop shows a copy of the saved skills taken when
    // the Divinity Statue menu opens (still being located), so writes at this point are ignored.
    if let Ok(mut snap) = SHOP_SNAPSHOT.write() {
        *snap = Some(flags);
    }
    log::debug!("Skill shop opened, showing purchase checks: {:X?}", flags);
}

/// Returns skills bought since the last call (from either the session or live copy)
pub(crate) fn take_bought_skills() -> Vec<&'static str> {
    let Ok(mut snap) = SHOP_SNAPSHOT.write() else {
        return vec![];
    };
    let Some(old) = *snap else {
        return vec![];
    };
    // Only the mission list the shop uses: Dante's saved and live lists hold received skills,
    // which aren't purchases
    let Some(current) = mission_expertise() else {
        return vec![];
    };
    let received = EXPECTED_EXPERTISE
        .read()
        .ok()
        .and_then(|e| *e)
        .unwrap_or(DEFAULT_SKILLS);
    let mut bought: Vec<&'static str> = newly_bought(&old, &current)
        .into_iter()
        .filter(|name| {
            let d = SKILLS_MAP.get(name).unwrap();
            received[d.index] & d.flag == 0
        })
        .collect();
    if let Ok(mut reported) = REPORTED_PURCHASES.write() {
        bought.retain(|name| !reported.contains(name));
        reported.extend(bought.iter().copied());
    }
    if !bought.is_empty() {
        log::debug!("Skill shop: bought {:?}", bought);
        *snap = Some(current);
    }
    bought
}

/// Leaves the shop: stop showing purchase checks and put received skills back
pub(crate) fn close_skill_shop(data: &ArchipelagoData) {
    start_purchase_watch();

    if let Ok(mut snap) = SHOP_SNAPSHOT.write() {
        *snap = None;
    }
    IN_SKILL_SHOP.store(false, Ordering::SeqCst);
    set_skills(data);
    log::debug!("Skill shop closed, received skills re-applied");
}

/// Test build: log the session and live skill flags
pub(crate) fn log_expertise(when: &str) {
    let session = SessionData::with_read(|s| s.expertise).ok();
    let live = CharacterData::with_read(|c| c.expertise).ok();
    log::debug!(
        "Skill flags {}: session {:X?}, live {:X?}, mission {:X?}",
        when, session, live, mission_expertise()
    );
}

/// Skill flags in the mission's active copy (where the shop seems to write purchases), if available
fn mission_expertise() -> Option<[u32; 8]> {
    crate::data::game_structs::ActiveMissionActorData::with_read(|a| a.expertise).ok()
}

/// Test build: called every frame, logs whenever any skill flag copy changes
pub(crate) fn watch_skill_flags() {
    static LAST: RwLock<Option<(Option<[u32; 8]>, Option<[u32; 8]>, Option<[u32; 8]>)>> =
        RwLock::new(None);
    let now = (
        SessionData::with_read(|s| s.expertise).ok(),
        CharacterData::with_read(|c| c.expertise).ok(),
        mission_expertise(),
    );
    let Ok(mut last) = LAST.write() else {
        return;
    };
    if *last != Some(now) {
        log::debug!(
            "Skill flags changed: session {:X?}, live {:X?}, mission {:X?}",
            now.0, now.1, now.2
        );
        *last = Some(now);
    }
}

/// After the shop closes the game writes bought skills into the mission copy about a second
/// later. (flags at shop close, when to stop watching)
/// Purchases already turned into checks this session
static REPORTED_PURCHASES: LazyLock<RwLock<std::collections::HashSet<&'static str>>> =
    LazyLock::new(|| RwLock::new(std::collections::HashSet::new()));
static PENDING_PURCHASES: RwLock<Option<([u32; 8], std::time::Instant)>> = RwLock::new(None);
const PURCHASE_WATCH: std::time::Duration = std::time::Duration::from_secs(3);

/// Puts the mission's "right now" and "next start" skill lists back to the received skills
fn restore_mission_skills(why: &str) {
    if IN_SKILL_SHOP.load(Ordering::SeqCst) {
        return;
    }
    let Some(received) = EXPECTED_EXPERTISE.read().ok().and_then(|e| *e) else {
        return;
    };
    let _ = crate::data::game_structs::ActiveMissionActorData::with_mut(|a| a.expertise = received);
    let _ = crate::data::game_structs::QueuedMissionActorData::with_mut(|q| q.expertise = received);
    log::debug!("Skill shop: mission skill lists restored to received skills ({})", why);
}

fn start_purchase_watch() {
    let start = mission_expertise().unwrap_or(DEFAULT_SKILLS);
    if let Ok(mut pending) = PENDING_PURCHASES.write() {
        *pending = Some((start, std::time::Instant::now() + PURCHASE_WATCH));
    }
}

/// Called every frame: skills newly written into the mission copy after the shop closed, not
/// counting skills that were received as items
pub(crate) fn poll_bought_skills() -> Vec<&'static str> {
    let Ok(mut pending) = PENDING_PURCHASES.try_write() else {
        return vec![];
    };
    let Some((prev, until)) = *pending else {
        return vec![];
    };
    if std::time::Instant::now() > until {
        *pending = None;
        restore_mission_skills("watch ended");
        return vec![];
    }
    let Some(now) = mission_expertise() else {
        return vec![];
    };
    // The game writes "the saved skills it copied + skills bought this visit". Anything there
    // that wasn't received as an item is a purchase; ones already sent are skipped.
    let received = EXPECTED_EXPERTISE
        .read()
        .ok()
        .and_then(|e| *e)
        .unwrap_or(DEFAULT_SKILLS);
    let Ok(mut reported) = REPORTED_PURCHASES.write() else {
        return vec![];
    };
    let bought: Vec<&'static str> = SKILLS_MAP
        .iter()
        .filter(|(name, d)| {
            now[d.index] & d.flag != 0
                && received[d.index] & d.flag == 0
                && !reported.contains(*name)
        })
        .map(|(name, _)| *name)
        .collect();
    reported.extend(bought.iter().copied());
    drop(reported);
    if now != prev {
        // The game has written the purchase: put both mission lists straight back to the
        // skills actually received
        *pending = None;
        restore_mission_skills("purchase read");
    }
    if !bought.is_empty() {
        log::debug!("Skill shop: bought {:?} (written after the shop closed)", bought);
    }
    bought
}

// ---------------------------------------------------------------------------------------------
// Test build only: finds where the shop keeps the skill list it shows. Read-only: it searches
// game memory for skill lists and logs where they are and when they change. Nothing here is
// used to send checks or written back to the game.

/// Skill lists found when the shop last opened
static SKILL_COPY_ADDRS: RwLock<Vec<usize>> = RwLock::new(Vec::new());
/// Last logged values of those lists
static SKILL_COPY_VALUES: RwLock<Vec<Option<[u32; 8]>>> = RwLock::new(Vec::new());

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

/// Where an address sits, relative to the game structs we know
fn describe_addr(addr: usize, shop: usize) -> String {
    let module = *crate::DMC3_ADDRESS;
    let mission = if crate::data::game_structs::MissionData::is_valid() {
        randomizer_utilities::read_data_from_address::<usize>(
            crate::data::game_structs::MissionData::ptr(),
        )
    } else {
        0
    };
    let mut parts = vec![];
    for (label, base, range) in [
        ("game", module, 0x2000000usize),
        ("mission", mission, 0x10000),
        ("shop", shop, 0x10000),
    ] {
        if base != 0 && addr >= base && addr < base + range {
            parts.push(format!("{}+{:#X}", label, addr - base));
        }
    }
    parts.join(", ")
}

/// Test build: searches all of the game's memory (safely) for skill lists (words 4 and 5 never
/// change, so they mark one) and logs them. Read-only. Takes a few seconds.
pub(crate) fn find_skill_copies(when: &str, shop: usize) {
    use windows::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
    };
    const CHUNK: usize = 0x10_0000;
    let started = std::time::Instant::now();
    let dll_base = find_skill_copies as usize & !0xFFFF_FFFF; // skip the mod's own memory roughly
    let mut found: Vec<usize> = vec![];
    let mut buf = vec![0u8; CHUNK + 32];
    let mut addr: usize = 0x10000;
    while addr < 0x7FFF_FFFF_0000 && found.len() < 128 {
        let mut info = MEMORY_BASIC_INFORMATION::default();
        let size = unsafe {
            VirtualQuery(Some(addr as *const _), &mut info, size_of::<MEMORY_BASIC_INFORMATION>())
        };
        if size == 0 {
            break;
        }
        let base = info.BaseAddress as usize;
        let region_end = base + info.RegionSize;
        if info.State == MEM_COMMIT
            && info.Protect.0 != 0
            && info.Protect.0 & (PAGE_GUARD.0 | PAGE_NOACCESS.0) == 0
            && base & !0xFFFF_FFFF != dll_base
        {
            let mut p = base;
            while p < region_end && found.len() < 128 {
                let len = (region_end - p).min(CHUNK);
                if len >= 32 && safe_read(p, &mut buf[..len]) {
                    let mut i = 0;
                    while i + 32 <= len {
                        let w4 = u32::from_le_bytes(buf[i + 16..i + 20].try_into().unwrap());
                        if w4 == DEFAULT_SKILLS[4]
                            && u32::from_le_bytes(buf[i + 20..i + 24].try_into().unwrap())
                                == DEFAULT_SKILLS[5]
                        {
                            found.push(p + i);
                        }
                        i += 4;
                    }
                }
                p += len;
            }
        }
        addr = region_end.max(addr + 0x1000);
    }
    log::debug!(
        "Skill list search ({}): {} found in {:?}",
        when,
        found.len(),
        started.elapsed()
    );
    let mut values = vec![];
    for &a in &found {
        let v = read_copy(a);
        log::debug!("  {:#X} [{}]: {:X?}", a, describe_addr(a, shop), v);
        values.push(v);
    }
    find_pointers_to(&found);
    if let Ok(mut addrs) = SKILL_COPY_ADDRS.write() {
        *addrs = found;
    }
    if let Ok(mut vals) = SKILL_COPY_VALUES.write() {
        *vals = values;
    }
}

/// Test build: looks through the game's own data (the exe's memory) for pointers to, or up to
/// 0x8000 bytes before, each skill list found, so it can be found again after a relaunch.
/// Read-only.
fn find_pointers_to(found: &[usize]) {
    use windows::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
    };
    const RANGE: usize = 0x8000;
    let module = *crate::DMC3_ADDRESS;
    let targets: Vec<usize> = found
        .iter()
        .copied()
        .filter(|&a| !(a >= module && a < module + 0x4000000))
        .collect();
    if targets.is_empty() {
        return;
    }
    let started = std::time::Instant::now();
    let mut hits: Vec<String> = vec![];
    let mut buf = vec![0u8; 0x10_0000];
    let mut addr = module;
    while addr < module + 0x4000000 && hits.len() < 200 {
        let mut info = MEMORY_BASIC_INFORMATION::default();
        let size = unsafe {
            VirtualQuery(Some(addr as *const _), &mut info, size_of::<MEMORY_BASIC_INFORMATION>())
        };
        if size == 0 || info.AllocationBase as usize != module {
            break;
        }
        let base = info.BaseAddress as usize;
        let region_end = base + info.RegionSize;
        if info.State == MEM_COMMIT
            && info.Protect.0 != 0
            && info.Protect.0 & (PAGE_GUARD.0 | PAGE_NOACCESS.0) == 0
        {
            let mut p = base;
            while p < region_end {
                let len = (region_end - p).min(buf.len());
                if len >= 8 && safe_read(p, &mut buf[..len]) {
                    let mut i = 0;
                    while i + 8 <= len {
                        let v = u64::from_le_bytes(buf[i..i + 8].try_into().unwrap()) as usize;
                        for &t in &targets {
                            if v <= t && t - v < RANGE {
                                hits.push(format!(
                                    "game+{:#X} = {:#X} (list at +{:#X})",
                                    p + i - module,
                                    v,
                                    t - v
                                ));
                            }
                        }
                        i += 8;
                    }
                }
                p += len;
            }
        }
        addr = region_end;
    }
    log::debug!(
        "Pointers to skill lists in game data ({} found in {:?}):",
        hits.len(),
        started.elapsed()
    );
    for h in hits {
        log::debug!("  {}", h);
    }
}

/// Called every frame: logs any found skill list whose value changed
pub(crate) fn watch_skill_copies() {
    let Ok(addrs) = SKILL_COPY_ADDRS.try_read() else {
        return;
    };
    let Ok(mut vals) = SKILL_COPY_VALUES.try_write() else {
        return;
    };
    for (i, &a) in addrs.iter().enumerate() {
        let now = read_copy(a);
        if vals.get(i) != Some(&now) {
            // Only log while it still looks like a skill list (scratch memory gets reused)
            if let Some(w) = now
                && w[4] == DEFAULT_SKILLS[4]
                && w[5] == DEFAULT_SKILLS[5]
            {
                log::debug!("Skill list {:#X} changed: {:X?}", a, w);
            }
            if let Some(v) = vals.get_mut(i) {
                *v = now;
            }
        }
    }
}
