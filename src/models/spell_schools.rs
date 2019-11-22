use serde::{Serialize, Deserialize};

use crate::models::schools::Schools;
use crate::models::spell::Spell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellSchools {
    pub name:           String,
    pub spell_duration: String,
    pub options:        Vec<String>,
    pub reach:          String,
    pub difficulty:     String,
    pub typus:          String,
    pub enforced:       String,
    pub effect:         String,
    pub cast_duration:  String,
    pub cost:           String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schools:        Option<_SpellSchools>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _SpellSchools {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire:          Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection:    Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light:         Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection:     Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strengthening: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metamorphism:  Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bann:          Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illusion:      Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement:      Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind:          Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heal:          Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death:         Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fate:          Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature:        Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control:       Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fight:         Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water:         Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow:        Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earth:         Option<i32>
}

impl SpellSchools {
    pub fn from_left_join(data: (Spell, Option<Schools>)) -> SpellSchools {
        let schools: Option<_SpellSchools> = match data.1 {
            Some(s) => Some(_SpellSchools {
                fire:          s.fire,
                protection:    s.protection,
                light:         s.light,
                detection:     s.detection,
                strengthening: s.strengthening,
                metamorphism:  s.metamorphism,
                bann:          s.bann,
                illusion:      s.illusion,
                movement:      s.movement,
                wind:          s.wind,
                heal:          s.heal,
                death:         s.death,
                fate:          s.fate,
                nature:        s.nature,
                control:       s.control,
                fight:         s.fight,
                water:         s.water,
                shadow:        s.shadow,
                earth:         s.earth,
            }),
            None => None
        };

        SpellSchools {
            name:           data.0.name,
            spell_duration: data.0.spell_duration,
            options:        data.0.options,
            reach:          data.0.reach,
            difficulty:     data.0.difficulty,
            typus:          data.0.typus,
            enforced:       data.0.enforced,
            effect:         data.0.effect,
            cast_duration:  data.0.cast_duration,
            cost:           data.0.cost,
            schools:        schools
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
