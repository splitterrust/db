/*
This is the general spell data with the corresponding schools linked.
*/
use serde::{
    Deserialize,
    Serialize,
};

use crate::models::schools::Schools as _Schools;
use crate::models::spell::Spell as _Spell;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell {
    pub cast_duration:      String,
    pub options:            Vec<String>,
    pub range:              String,
    pub difficulty:         String,
    pub typus:              String,
    pub enforced:           String,
    pub effect:             String,
    pub duration_of_effect: String,
    pub cost:               String,
    pub name:               String,
    pub schools:            Schools,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schools {
    pub fire:          Option<i32>,
    pub protection:    Option<i32>,
    pub light:         Option<i32>,
    pub detection:     Option<i32>,
    pub strengthening: Option<i32>,
    pub metamorphism:  Option<i32>,
    pub bann:          Option<i32>,
    pub illusion:      Option<i32>,
    pub movement:      Option<i32>,
    pub wind:          Option<i32>,
    pub heal:          Option<i32>,
    pub death:         Option<i32>,
    pub fate:          Option<i32>,
    pub nature:        Option<i32>,
    pub control:       Option<i32>,
    pub fight:         Option<i32>,
    pub water:         Option<i32>,
    pub shadow:        Option<i32>,
    pub earth:         Option<i32>,
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
            schools,
        }
    }

    pub fn to_json_pretty(&self) -> String {
        use serde_json::json;
        serde_json::to_string_pretty(&json!(self)).unwrap()
    }

    pub fn to_json(&self) -> String {
        use serde_json::json;
        json!(self).to_string()
    }
}
