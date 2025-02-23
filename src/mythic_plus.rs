use askama_axum::Template;

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
    fn rgb(&self) -> &'static str {
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

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub class: PlayerClass,
    pub realm: &'static str,
}

#[derive(Template)]
#[template(path = "mythic-plus.html")]
pub struct RaidFramesTemplate {
    pub show_noti: bool,
    pub players: Vec<PlayerDisplay>,
}

#[derive(Debug)]
pub struct PlayerDisplay {
    pub name: &'static str,
    pub class_rgb: &'static str,
    pub realm: &'static str,
}

pub async fn mythicplus_page() -> axum::response::Html<String> {
    // Creating the list of players, including class RGB
    let players = vec![
        Player {
            name: "Captclit",
            class: PlayerClass::Monk,
            realm: "Thrall",
        },
        Player {
            name: "Rogermeta",
            class: PlayerClass::DemonHunter,
            realm: "Stormrage",
        },
        Player {
            name: "Mistladora",
            class: PlayerClass::Evoker,
            realm: "Stormrage",
        },
        Player {
            name: "Evelianne",
            class: PlayerClass::Shaman,
            realm: "Stormrage",
        },
        Player {
            name: "Delusionol",
            class: PlayerClass::Priest,
            realm: "Stormrage",
        },
        Player {
            name: "Sylvána",
            class: PlayerClass::Priest,
            realm: "Stormrage",
        },
        Player {
            name: "Shdo",
            class: PlayerClass::Paladin,
            realm: "Stormrage",
        },
        Player {
            name: "Amashirochan",
            class: PlayerClass::Paladin,
            realm: "Stormrage",
        },
        Player {
            name: "Jakksparrow",
            class: PlayerClass::Paladin,
            realm: "Stormrage",
        },
        Player {
            name: "Amarelysa",
            class: PlayerClass::DemonHunter,
            realm: "Stormrage",
        },
        Player {
            name: "Nuzzler",
            class: PlayerClass::Druid,
            realm: "Stormrage",
        },
        Player {
            name: "Kaelirious",
            class: PlayerClass::Hunter,
            realm: "Stormrage",
        },
        Player {
            name: "Chillfi",
            class: PlayerClass::Mage,
            realm: "Stormrage",
        },
        Player {
            name: "Lanathallan",
            class: PlayerClass::Warlock,
            realm: "Stormrage",
        },
        Player {
            name: "Chuubers",
            class: PlayerClass::Warrior,
            realm: "Stormrage",
        },
        Player {
            name: "Nicechint",
            class: PlayerClass::Rogue,
            realm: "Stormrage",
        },
        Player {
            name: "Tusknight",
            class: PlayerClass::DeathKnight,
            realm: "Stormrage",
        },
    ];


    // Mapping Player struct to PlayerDisplay to include class_rgb
    let player_display: Vec<PlayerDisplay> = players.into_iter().map(|player| {
        PlayerDisplay {
            name: player.name,
            class_rgb: player.class.rgb(),
            realm: player.realm,
        }
    }).collect();

    // Rendering the template with the player data
    let template = RaidFramesTemplate { show_noti: true, players: player_display };
    let rendered = template.render().unwrap();
    axum::response::Html(rendered)
}

