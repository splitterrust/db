use super::types::Spell as Jspell;

use std::error::Error;
use std::fs;
use serde_json::from_str;
use crate::{write_schools, write_spell, NewSchools, NewSpell};

fn read_spells_from_file(path: &str) -> Result<Vec<Jspell>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)
        .expect("Failed to read File");

    let spells: Vec<Jspell> = from_str(&contents)?;

    Ok(spells)
}

pub fn write_json_spells_to_db() {
    let spells = read_spells_from_file("./src/json/spells_new.json").unwrap();
    for spell in &spells {
        let new_school = NewSchools {
            fire:          spell.schools.fire.as_ref(),
            protection:    spell.schools.protection.as_ref(),
            ligth:         spell.schools.light.as_ref(),
            detection:     spell.schools.detection.as_ref(),
            strengthening: spell.schools.strengthening.as_ref(),
            metamorphism:  spell.schools.metamorphism.as_ref(),
            bann:          spell.schools.bann.as_ref(),
            illusion:      spell.schools.illusion.as_ref(),
            movement:      spell.schools.movement.as_ref(),
            wind:          spell.schools.wind.as_ref(),
            heal:          spell.schools.heal.as_ref(),
            death:         spell.schools.death.as_ref(),
            fate:          spell.schools.fate.as_ref(),
            nature:        spell.schools.nature.as_ref(),
            control:       spell.schools.control.as_ref(),
            fight:         spell.schools.fight.as_ref(),
            water:         spell.schools.water.as_ref(),
            shadow:        spell.schools.shadow.as_ref(),
            earth:         spell.schools.earth.as_ref()
        };
        let school = write_schools(&new_school);

        let new_spell = NewSpell {
            name:           &spell.name,
            spell_duration: &spell.spell_duration,
            options:        &spell.options,
            reach:          &spell.reach,
            difficulty:     &spell.difficulty,
            typus:          &spell.typus,
            enforced:       &spell.enforced,
            effect:         &spell.effect,
            cast_duration:  &spell.cast_duration,
            cost:           &spell.cost,
            schools_id:     &school.id
        };
        write_spell(&new_spell);
    }
}