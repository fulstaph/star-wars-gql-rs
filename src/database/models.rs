#[derive(Debug)]
pub struct Starship {
    pub id: i64,
    pub name: String,
    pub class: String,
}

#[derive(Debug)]
pub struct Planet {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub director_id: i64,
    pub scriptwriter_id: i64,
    pub producer_id: i64,
    pub release_date: sqlx::types::chrono::NaiveDate,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "race", rename_all = "lowercase")]
pub enum Race {
    Human,
    Droid,
    Wookie,
}

#[derive(Debug)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub race: Race,
    pub starship_id: i64,
}

#[derive(sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "profession", rename_all = "lowercase")]
pub enum Profession {
    Director,
    Scriptwriter,
    Producer,
    Cinematographer,
}

#[derive(Debug, Clone)]
pub struct Filmmaker {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub profession: Profession,
}
