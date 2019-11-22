table! {
    schools (id) {
        id -> Int4,
        fire -> Nullable<Int4>,
        protection -> Nullable<Int4>,
        light -> Nullable<Int4>,
        detection -> Nullable<Int4>,
        strengthening -> Nullable<Int4>,
        metamorphism -> Nullable<Int4>,
        bann -> Nullable<Int4>,
        illusion -> Nullable<Int4>,
        movement -> Nullable<Int4>,
        wind -> Nullable<Int4>,
        heal -> Nullable<Int4>,
        death -> Nullable<Int4>,
        fate -> Nullable<Int4>,
        nature -> Nullable<Int4>,
        control -> Nullable<Int4>,
        fight -> Nullable<Int4>,
        water -> Nullable<Int4>,
        shadow -> Nullable<Int4>,
        earth -> Nullable<Int4>,
    }
}

table! {
    spells (id) {
        id -> Int4,
        name -> Varchar,
        spell_duration -> Varchar,
        options -> Array<Text>,
        reach -> Varchar,
        difficulty -> Varchar,
        typus -> Varchar,
        enforced -> Varchar,
        effect -> Text,
        cast_duration -> Varchar,
        cost -> Varchar,
        schools_id -> Int4,
    }
}

joinable!(spells -> schools (schools_id));

allow_tables_to_appear_in_same_query!(
    schools,
    spells,
);
