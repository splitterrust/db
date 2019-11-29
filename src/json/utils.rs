use crate::models::spell_schools_pretty_ger::Spell;

use crate::{
    write_schools,
    write_spell,
    NewSchools,
    NewSpell,
};
use serde_json::from_str;
use std::error::Error;
use std::fs;

fn read_spells_from_file(path: &str) -> Result<Vec<Spell>, Box<dyn Error>> {
    let contents = fs::read_to_string(path).expect("Failed to read File");

    let spells: Vec<Spell> = from_str(&contents)?;

    Ok(spells)
}

pub fn write_json_spells_to_db() {
    let spells = read_spells_from_file("./src/json/spells_new.json").unwrap();
    for spell in &spells {
        let new_school = NewSchools {
            fire:          spell.schools.fire.as_ref(),
            protection:    spell.schools.protection.as_ref(),
            light:         spell.schools.light.as_ref(),
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
            earth:         spell.schools.earth.as_ref(),
        };
        let school = write_schools(&new_school);

        let new_spell = NewSpell {
            name:               &spell.name,
            cast_duration:      &spell.cast_duration,
            options:            &spell.options,
            range:              &spell.range,
            difficulty:         &spell.difficulty,
            typus:              &spell.typus,
            enforced:           &spell.enforced,
            effect:             &spell.effect,
            duration_of_effect: &spell.duration_of_effect,
            cost:               &spell.cost,
            schools_id:         &school.id,
        };
        write_spell(&new_spell);
    }
}
