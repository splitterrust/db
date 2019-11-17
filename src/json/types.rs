// This module contains the types from the json file from which the spells are
// derived.
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell {
    #[serde(rename = "Dauer")]
    pub spell_duration: String,
    #[serde(rename = "Optionen")]
    pub options: Vec<String>,
    #[serde(rename = "Reichweite")]
    pub reach: String,
    #[serde(rename = "Schwierigkeit")]
    pub difficulty: String,
    #[serde(rename = "Typus")]
    pub typus: String,
    #[serde(rename = "Verstärkt")]
    pub enforced: String,
    #[serde(rename = "Wirkung")]
    pub effect: String,
    #[serde(rename = "Wirkungsdauer")]
    pub cast_duration: String,
    #[serde(rename = "Zauberkosten")]
    pub cost: String,
    pub name: String,
    #[serde(rename = "schulen")]
    pub schools: Schools,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schools {
    #[serde(rename = "Feuermagie")]
    pub fire: Option<i32>,
    #[serde(rename = "Schutzmagie")]
    pub protection: Option<i32>,
    #[serde(rename = "Lichtmagie")]
    pub light: Option<i32>,
    #[serde(rename = "Erkenntnismagie")]
    pub detection: Option<i32>,
    #[serde(rename = "Stärkungsmagie")]
    pub strengthening: Option<i32>,
    #[serde(rename = "Verwandlungsmagie")]
    pub metamorphism: Option<i32>,
    #[serde(rename = "Bannmagie")]
    pub bann: Option<i32>,
    #[serde(rename = "Illusionsmagie")]
    pub illusion: Option<i32>,
    #[serde(rename = "Bewegungsmagie")]
    pub movement: Option<i32>,
    #[serde(rename = "Windmagie")]
    pub wind: Option<i32>,
    #[serde(rename = "Heilungsmagie")]
    pub heal: Option<i32>,
    #[serde(rename = "Todesmagie")]
    pub death: Option<i32>,
    #[serde(rename = "Schicksalsmagie")]
    pub fate: Option<i32>,
    #[serde(rename = "Naturmagie")]
    pub nature: Option<i32>,
    #[serde(rename = "Beherrschungsmagie")]
    pub control: Option<i32>,
    #[serde(rename = "Kampfmagie")]
    pub fight: Option<i32>,
    #[serde(rename = "Wassermagie")]
    pub water: Option<i32>,
    #[serde(rename = "Schattenmagie")]
    pub shadow: Option<i32>,
    #[serde(rename = "Felsmagie")]
    pub earth: Option<i32>,
}
