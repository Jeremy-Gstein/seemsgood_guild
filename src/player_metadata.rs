#[derive(Debug)]
pub enum PlayerRole {
    Tank,
    Healer,
    Dps,
}
impl PlayerRole {
    pub fn icon_url(&self) -> &'static str {
        match self {
            PlayerRole::Tank => "https://cdn.raiderio.net/assets/img/role_tank-6cee7610058306ba277e82c392987134.png",
            PlayerRole::Healer => "https://cdn.raiderio.net/assets/img/role_healer-984e5e9867d6508a714a9c878d87441b.png",
            PlayerRole::Dps => "https://cdn.raiderio.net/assets/img/role_dps-eb25989187d4d3ac866d609dc009f090.png",
        }
    }
}

impl std::fmt::Display for PlayerRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            PlayerRole::Tank => "Tank",
            PlayerRole::Healer => "Healer",
            PlayerRole::Dps => "Dps",
        };
        write!(f, "{}", role_str)
    }
}



#[derive(Debug)]
pub enum PlayerClass {
    Warrior,
    Mage,
    Rogue,
    Hunter,
    Druid,
    Paladin,
    Priest,
    Warlock,
    Monk,
    DeathKnight,
    Shaman,
    DemonHunter,
    Evoker,
}

impl PlayerClass {
    pub fn rgb(&self) -> &'static str {
        match self {
            PlayerClass::DeathKnight => "rgb(196, 30, 58)",
            PlayerClass::DemonHunter => "rgb(163, 48, 201)",
            PlayerClass::Druid => "rgb(255, 124, 10)",
            PlayerClass::Evoker => "rgb(51, 147, 127)",
            PlayerClass::Hunter => "rgb(170, 211, 114)",
            PlayerClass::Mage => "rgb(63, 199, 235)",
            PlayerClass::Monk => "rgb(0, 255, 152)",
            PlayerClass::Paladin => "rgb(244, 140, 186)",
            PlayerClass::Priest => "rgb(255, 255, 255)",
            PlayerClass::Rogue => "rgb(255, 244, 104)",
            PlayerClass::Shaman => "rgb(0, 112, 221)",
            PlayerClass::Warlock => "rgb(135, 136, 238)",
            PlayerClass::Warrior => "rgb(198, 155, 109)",
        }
    }
}

impl std::fmt::Display for PlayerClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_str = match self {
            PlayerClass::DeathKnight => "DeathKnight",
            PlayerClass::DemonHunter => "DemonHunter",
            PlayerClass::Druid => "Druid",
            PlayerClass::Evoker => "Evoker",
            PlayerClass::Hunter => "Hunter",
            PlayerClass::Mage => "Mage",
            PlayerClass::Monk => "Monk",
            PlayerClass::Paladin => "Paladin",
            PlayerClass::Priest => "Priest",
            PlayerClass::Rogue => "Rogue",
            PlayerClass::Shaman => "Shaman",
            PlayerClass::Warlock => "Warlock",
            PlayerClass::Warrior => "Warrior",
        };
        write!(f, "{}", class_str)
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub class: PlayerClass,
    pub realm: &'static str,
    pub role: PlayerRole,
}



pub fn build_roster() -> Vec<Player> {
    let players = vec![
        // Tanks
        Player {
            name: "Jtusendh",
            class: PlayerClass::DemonHunter,
            realm: "Stormrage",
            role: PlayerRole::Tank,
        },
        Player {
            name: "Paliduh",
            class: PlayerClass::Paladin,
            realm: "Stormrage",
            role: PlayerRole::Tank,
        },

        // Healers
        Player {
            name: "Delusionol",
            class: PlayerClass::Priest,
            realm: "Stormrage",
            role: PlayerRole::Healer,
        },
        Player {
            name: "Evelianne",
            class: PlayerClass::Monk,
            realm: "Stormrage",
            role: PlayerRole::Healer,
        },
        Player {
            name: "Pipmeow",
            class: PlayerClass::Druid,
            realm: "Tichondrius",
            role: PlayerRole::Healer,
        },
        Player {
            name: "Auraelia",
            class: PlayerClass::Priest,
            realm: "Zul'jin",
            role: PlayerRole::Healer,
        },
        Player {
            name: "Caael",
            class: PlayerClass::Paladin,
            realm: "Zul'jin",
            role: PlayerRole::Healer,
        },

        // DPS
        Player {
            name: "Infilicious",
            class: PlayerClass::Mage,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Hekthuzad",
            class: PlayerClass::Mage,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Indico",
            class: PlayerClass::Evoker,
            realm: "Zul'jin",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Ladöra",
            class: PlayerClass::Priest,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Lanathallan",
            class: PlayerClass::Warlock,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Notshodo",
            class: PlayerClass::Evoker,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Kolzane",
            class: PlayerClass::Hunter,
            realm: "Ysera",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Speara",
            class: PlayerClass::Druid,
            realm: "Kel'Thuzad",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Nyanslok",
            class: PlayerClass::Warlock,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Dubshamm",
            class: PlayerClass::Shaman,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Chuubers",
            class: PlayerClass::Warrior,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Conncrete",
            class: PlayerClass::Rogue,
            realm: "Tichondrius",
            role: PlayerRole::Dps,
        },
        Player {
            name: "Filio",
            class: PlayerClass::Monk,
            realm: "Stormrage",
            role: PlayerRole::Dps,
        },
    ];
    players
}


/* TODO use this api to get roster - 
 * curl -X 'GET' https://wowaudit.com/v1/characters -H 'accept: application/json' -H 'Authorization: c210cab6ac2405faca8e60e0514a94fddede5015902f9e52ec401a7a2086d970' |  jq '.[] | select(.rank == "Officer" or .rank == "Raider") | {name, realm, class, rank}'
*/

// pub fn build_roster() -> Vec<Player> {
//     // Creating the list of players, including class RGB
//     let players = vec![
//         // tanks
//         Player { name: "Jtusendh", class: PlayerClass::DemonHunter, realm: "Stormrage", role: PlayerRole::Tank, },
//         Player { name: "Paliduh", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Tank, },
//         // healers
//         Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage" },
//         Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage" },
//         Player { name: "Pipmeow", class: PlayerClass::Druid, realm: "Tichondrius" },
//         Player { name: "Auraelia", class: PlayerClass::Priest, realm: "Zul'jin" },
//         Player { name: "Caael", class: PlayerClass::Paladin, realm: "Zul'jin" },
//         // damage 
//         Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage" },
//         Player { name: "Hekthuzad", class: PlayerClass::Mage, realm: "Stormrage" },
//         Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin" },
//         Player { name: "Ladöra", class: PlayerClass::Priest, realm: "Stormrage" },
//         Player { name: "Lanathallan", class: PlayerClass::Warlock, realm: "Stormrage" },
//         Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage" },
//         Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera" },
//         Player { name: "Speara", class: PlayerClass::Druid, realm: "Kel'Thuzad" },
//         Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage" },
//         Player { name: "Dubshamm", class: PlayerClass::Shaman, realm: "Stormrage" },
//         Player { name: "Chuubers", class: PlayerClass::Warrior, realm: "Stormrage" },
//         Player { name: "Conncrete", class: PlayerClass::Rogue, realm: "Tichondrius" },
//         Player { name: "Filio", class: PlayerClass::Monk, realm: "Stormrage" },
//     ];
//     players
//
// }
