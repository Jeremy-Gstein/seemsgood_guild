use askama_axum::Template;

#[derive(Debug)]
enum PlayerClass {
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
struct Player {
    name: &'static str,
    class: PlayerClass,
    realm: &'static str,
}

#[derive(Template)]
#[template(path = "mythic-plus.html")]
struct RaidFramesTemplate {
    show_noti: bool,
    players: Vec<Player>,
}

pub async fn mythicplus_page() -> axum::response::Html<String> {
    // Creating the list of players, including class RGB
    let players = vec![
    Player { name: "Actuallypete", class: PlayerClass::Warrior, realm: "Zul'jin" },
    Player { name: "Auraelia", class: PlayerClass::Priest, realm: "Zul'jin" },
    Player { name: "Bigtittyrog", class: PlayerClass::Druid, realm: "Stormrage" },
    Player { name: "Caael", class: PlayerClass::Paladin, realm: "Zul'jin" },
    Player { name: "Chewsdayinit", class: PlayerClass::Shaman, realm: "Proudmoore" },
    Player { name: "Chuubers", class: PlayerClass::Warrior, realm: "Stormrage" },
    Player { name: "Crsete", class: PlayerClass::Rogue, realm: "Tichondrius" },
    Player { name: "Crypticist", class: PlayerClass::DeathKnight, realm: "Zul'jin" },
    Player { name: "Delusionil", class: PlayerClass::Priest, realm: "Stormrage" },
    Player { name: "Emlay", class: PlayerClass::Priest, realm: "Sargeras" },
    Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage" },
    Player { name: "Filio", class: PlayerClass::Monk, realm: "Stormrage" },
    Player { name: "Fliptwisty", class: PlayerClass::DemonHunter, realm: "Stormrage" },
    Player { name: "Hekthuzad", class: PlayerClass::Mage, realm: "Stormrage" },
    Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin" },
    Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage" },
    Player { name: "Jakksparrow", class: PlayerClass::Paladin, realm: "Stormrage" },
    Player { name: "Juukmonk", class: PlayerClass::Monk, realm: "Zul'jin" },
    Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera" },
    Player { name: "Ladöra", class: PlayerClass::Priest, realm: "Stormrage" },
    Player { name: "Lanathallan", class: PlayerClass::Warlock, realm: "Stormrage" },
    Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage" },
    Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage" },
    Player { name: "Ohioforluvrz", class: PlayerClass::DeathKnight, realm: "Stormrage" },
    Player { name: "Paliduh", class: PlayerClass::Paladin, realm: "Stormrage" },
    Player { name: "Pipmeow", class: PlayerClass::Druid, realm: "Tichondrius" },
    Player { name: "Ppdx", class: PlayerClass::Rogue, realm: "Stormrage" },
    Player { name: "Quelstyle", class: PlayerClass::Paladin, realm: "Zul'jin" },
    Player { name: "Speara", class: PlayerClass::Druid, realm: "Kel'Thuzad" },
    Player { name: "Stepdoon", class: PlayerClass::Paladin, realm: "Proudmoore" },
    Player { name: "Vinnea", class: PlayerClass::Shaman, realm: "Kel'Thuzad" },
    Player { name: "Volkzero", class: PlayerClass::Shaman, realm: "Emerald Dream" }
    ];



    // Rendering the template with the player data
    let template = RaidFramesTemplate { show_noti: true, players };
    let rendered = template.render().unwrap();
    axum::response::Html(rendered)
}
