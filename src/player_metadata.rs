#[derive(Debug)]
pub enum PlayerRole {
    Tank,
    Healer,
    Dps,
}

impl PlayerRole {
    pub fn icon_url(&self) -> &'static str {
        match self {
            PlayerRole::Tank => "https://r2.seemsgood.org/content/icons/tank.png",
            PlayerRole::Healer => "https://r2.seemsgood.org/content/icons/healer.png",
            PlayerRole::Dps => "https://r2.seemsgood.org/content/icons/dps.png",
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


// Roster for specific s2 mythic gallywix kill.
// in future this will take a .json from wowaudit api to translate roster -> Player struct fields
pub fn build_roster() -> Vec<Player> {
    let players = vec![
        // Tanks
        Player { name: "Whare", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Tank },
        Player { name: "Jaemsy", class: PlayerClass::Warrior, realm: "Stormrage", role: PlayerRole::Tank },
        // Healers
        Player { name: "Pipmeow", class: PlayerClass::Druid, realm: "Tichondrius", role: PlayerRole::Healer },
        Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Oldmanzand", class: PlayerClass::Shaman, realm: "Illidan", role: PlayerRole::Healer },
        // DPS
        Player { name: "Obiscuit", class: PlayerClass::DeathKnight, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Jedh", class: PlayerClass::DemonHunter, realm: "Dalaran", role: PlayerRole::Dps },
        Player { name: "Nuzzler", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Rogergrowth", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin", role: PlayerRole::Dps },
        Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera", role: PlayerRole::Dps },
        Player { name: "Stormßreeð", class: PlayerClass::Hunter, realm: "Thrall", role: PlayerRole::Dps },
        Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Filio", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Ppdx", class: PlayerClass::Rogue, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Dubshamm", class: PlayerClass::Shaman, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Aphitari", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Dps },
    ];
    players

}


/* TODO use this api to get roster - 
 * curl -X 'GET' https://wowaudit.com/v1/characters -H 'accept: application/json' -H 'Authorization: $KEY' |  jq '.[] | select(.rank == "Officer" or .rank == "Raider") | {name, realm, class, rank}'
*/
