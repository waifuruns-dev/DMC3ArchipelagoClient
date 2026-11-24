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
/// What the shop was shown when it opened (finished purchase checks only)
static SHOP_OPEN_FLAGS: RwLock<Option<[u32; 8]>> = RwLock::new(None);
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
    // Only the "next start" list, which the shop shows from. Dante's saved and live skills are
    // never given purchase checks.
    let _ = crate::data::game_structs::QueuedMissionActorData::with_mut(|q| q.expertise = flags);
    // The shop shows from the mission's "right now" list (received skills showed as learned).
    // Put back to received skills about a second after the shop closes.
    let _ = crate::data::game_structs::ActiveMissionActorData::with_mut(|a| a.expertise = flags);
    if let Ok(mut open) = SHOP_OPEN_FLAGS.write() {
        *open = Some(flags);
    }
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
    let mut bought = newly_bought(&old, &current);
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
    // The game writes "what the shop showed + skills bought this visit". Anything new is a
    // purchase (even a skill already received as an item); ones already sent are skipped.
    let shown = SHOP_OPEN_FLAGS
        .read()
        .ok()
        .and_then(|f| *f)
        .unwrap_or(DEFAULT_SKILLS);
    // The shop's write keeps everything it showed. If the list was replaced some other way
    // (e.g. a mission restart reloading received skills), it isn't a purchase.
    let from_shop = (0..8).all(|i| now[i] & shown[i] == shown[i]);
    let Ok(mut reported) = REPORTED_PURCHASES.write() else {
        return vec![];
    };
    let bought: Vec<&'static str> = SKILLS_MAP
        .iter()
        .filter(|(name, d)| {
            from_shop
                && now[d.index] & d.flag != 0
                && shown[d.index] & d.flag == 0
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

fn read_copy(addr: usize) -> Option<[u32; 8]> {
    if readable_len(addr, 32) == 32 {
        Some(unsafe { std::ptr::read_unaligned(addr as *const [u32; 8]) })
    } else {
        None
    }
}

/// Readable length from `addr`, at most `len` (stops at the end of the memory block)
fn readable_len(addr: usize, len: usize) -> usize {
    use windows::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
    };
    let mut info = MEMORY_BASIC_INFORMATION::default();
    let size = unsafe {
        VirtualQuery(Some(addr as *const _), &mut info, size_of::<MEMORY_BASIC_INFORMATION>())
    };
    if size == 0
        || info.State != MEM_COMMIT
        || info.Protect.0 == 0
        || info.Protect.0 & (PAGE_GUARD.0 | PAGE_NOACCESS.0) != 0
    {
        return 0;
    }
    let end = info.BaseAddress as usize + info.RegionSize;
    len.min(end.saturating_sub(addr))
}

/// The small areas to search: the skill shop's own data, the mission data and Dante's data
fn search_areas(shop: usize) -> Vec<(&'static str, usize, usize)> {
    let mut areas = vec![];
    if shop != 0 {
        areas.push(("shop", shop, 0x8000));
    }
    if crate::data::game_structs::MissionData::is_valid() {
        let m = randomizer_utilities::read_data_from_address::<usize>(
            crate::data::game_structs::MissionData::ptr(),
        );
        if m != 0 {
            areas.push(("mission", m, 0x2000));
        }
    }
    if CharacterData::is_valid() {
        let c = randomizer_utilities::read_data_from_address::<usize>(CharacterData::ptr());
        if c != 0 {
            areas.push(("character", c, 0x10000));
        }
    }
    areas
}

/// Shop data when it opened, to spot single flags the game turns on while buying
static SHOP_DATA_SNAPSHOT: RwLock<Option<(usize, Vec<u32>)>> = RwLock::new(None);

fn read_words(addr: usize, len: usize) -> Vec<u32> {
    let len = readable_len(addr, len) / 4 * 4;
    (0..len / 4)
        .map(|i| unsafe { std::ptr::read((addr + i * 4) as *const u32) })
        .collect()
}

/// Searches the shop, mission and Dante data for skill lists (words 4 and 5 never change, so
/// they mark one). Read-only, a few KB, so it's instant.
pub(crate) fn find_skill_copies(when: &str, shop: usize) {
    let mut found: Vec<usize> = vec![];
    for (label, base, len) in search_areas(shop) {
        let words = read_words(base, len);
        for i in 0..words.len().saturating_sub(7) {
            if words[i + 4] == DEFAULT_SKILLS[4] && words[i + 5] == DEFAULT_SKILLS[5] {
                let addr = base + i * 4;
                log::debug!(
                    "Skill list ({}) at {}+{:#X}: {:X?}",
                    when,
                    label,
                    i * 4,
                    &words[i..i + 8]
                );
                found.push(addr);
            }
        }
        log::debug!("Searched {} ({:#X}, {:#X} bytes readable)", label, base, words.len() * 4);
    }
    if shop != 0 && let Ok(mut snap) = SHOP_DATA_SNAPSHOT.write() {
        *snap = Some((shop, read_words(shop, 0x8000)));
    }
    let values = found.iter().map(|&a| read_copy(a)).collect();
    if let Ok(mut addrs) = SKILL_COPY_ADDRS.write() {
        *addrs = found;
    }
    if let Ok(mut vals) = SKILL_COPY_VALUES.write() {
        *vals = values;
    }
}

/// Logs spots in the shop's data where a single flag was turned on since it opened
pub(crate) fn log_shop_data_changes(when: &str) {
    let Ok(snap) = SHOP_DATA_SNAPSHOT.read() else {
        return;
    };
    let Some((shop, old)) = snap.as_ref() else {
        return;
    };
    let now = read_words(*shop, old.len() * 4);
    let mut changes = vec![];
    for (i, (o, n)) in old.iter().zip(now.iter()).enumerate() {
        let added = n & !o;
        if o != n && n & o == *o && added.count_ones() == 1 {
            changes.push(format!("+{:#X}: {:X} -> {:X}", i * 4, o, n));
        }
    }
    if !changes.is_empty() {
        log::debug!("Shop data flags turned on ({}): {}", when, changes.join(", "));
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
            log::debug!("Skill list {:#X} changed: {:X?}", a, now);
            if let Some(v) = vals.get_mut(i) {
                *v = now;
            }
        }
    }
}
