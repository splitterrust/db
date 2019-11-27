use serde::{
    Deserialize,
    Serialize,
};

use crate::models::schools::Schools as _Schools;
use crate::models::spell::Spell as _Spell;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell {
    #[serde(rename = "Dauer")]
    pub cast_duration: String,
    #[serde(rename = "Optionen")]
    pub options: Vec<String>,
    #[serde(rename = "Reichweite")]
    pub range: String,
    #[serde(rename = "Schwierigkeit")]
    pub difficulty: String,
    #[serde(rename = "Typus")]
    pub typus: String,
    #[serde(rename = "Verstärkt")]
    pub enforced: String,
    #[serde(rename = "Wirkung")]
    pub effect: String,
    #[serde(rename = "Wirkungsdauer")]
    pub duration_of_effect: String,
    #[serde(rename = "Zauberkosten")]
    pub cost: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Schulen")]
    pub schools: Schools,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schools {
    #[serde(rename = "Feuermagie", skip_serializing_if = "Option::is_none")]
    pub fire: Option<i32>,
    #[serde(rename = "Schutzmagie", skip_serializing_if = "Option::is_none")]
    pub protection: Option<i32>,
    #[serde(rename = "Lichtmagie", skip_serializing_if = "Option::is_none")]
    pub light: Option<i32>,
    #[serde(rename = "Erkenntnismagie", skip_serializing_if = "Option::is_none")]
    pub detection: Option<i32>,
    #[serde(rename = "Stärkungsmagie", skip_serializing_if = "Option::is_none")]
    pub strengthening: Option<i32>,
    #[serde(rename = "Verwandlungsmagie", skip_serializing_if = "Option::is_none")]
    pub metamorphism: Option<i32>,
    #[serde(rename = "Bannmagie", skip_serializing_if = "Option::is_none")]
    pub bann: Option<i32>,
    #[serde(rename = "Illusionsmagie", skip_serializing_if = "Option::is_none")]
    pub illusion: Option<i32>,
    #[serde(rename = "Bewegungsmagie", skip_serializing_if = "Option::is_none")]
    pub movement: Option<i32>,
    #[serde(rename = "Windmagie", skip_serializing_if = "Option::is_none")]
    pub wind: Option<i32>,
    #[serde(rename = "Heilungsmagie", skip_serializing_if = "Option::is_none")]
    pub heal: Option<i32>,
    #[serde(rename = "Todesmagie", skip_serializing_if = "Option::is_none")]
    pub death: Option<i32>,
    #[serde(rename = "Schicksalsmagie", skip_serializing_if = "Option::is_none")]
    pub fate: Option<i32>,
    #[serde(rename = "Naturmagie", skip_serializing_if = "Option::is_none")]
    pub nature: Option<i32>,
    #[serde(rename = "Beherrschungsmagie", skip_serializing_if = "Option::is_none")]
    pub control: Option<i32>,
    #[serde(rename = "Kampfmagie", skip_serializing_if = "Option::is_none")]
    pub fight: Option<i32>,
    #[serde(rename = "Wassermagie", skip_serializing_if = "Option::is_none")]
    pub water: Option<i32>,
    #[serde(rename = "Schattenmagie", skip_serializing_if = "Option::is_none")]
    pub shadow: Option<i32>,
    #[serde(rename = "Felsmagie", skip_serializing_if = "Option::is_none")]
    pub earth: Option<i32>,
}

impl Spell {
    pub fn from_left_join(data: (_Spell, _Schools)) -> Spell {
        let schools = Schools {
            fire:          data.1.fire,
            protection:    data.1.protection,
            light:         data.1.light,
            detection:     data.1.detection,
            strengthening: data.1.strengthening,
            metamorphism:  data.1.metamorphism,
            bann:          data.1.bann,
            illusion:      data.1.illusion,
            movement:      data.1.movement,
            wind:          data.1.wind,
            heal:          data.1.heal,
            death:         data.1.death,
            fate:          data.1.fate,
            nature:        data.1.nature,
            control:       data.1.control,
            fight:         data.1.fight,
            water:         data.1.water,
            shadow:        data.1.shadow,
            earth:         data.1.earth,
        };

        Spell {
            name:               data.0.name,
            duration_of_effect: data.0.duration_of_effect,
            options:            data.0.options,
            range:              data.0.range,
            difficulty:         data.0.difficulty,
            typus:              data.0.typus,
            enforced:           data.0.enforced,
            effect:             data.0.effect,
            cast_duration:      data.0.cast_duration,
            cost:               data.0.cost,
            schools:            schools,
        }
    }

    pub fn to_json_pretty(self) -> String {
        use serde_json::json;
        serde_json::to_string_pretty(&json!(self)).unwrap()
    }

    pub fn to_json(self) -> String {
        use serde_json::json;
        json!(self).to_string()
    }
}
