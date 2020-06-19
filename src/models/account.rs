pub struct Account {
    pub id: i32,
    pub name: String,
    pub last_character: String,
}

pub struct NewAccount<'a> {
    pub name: &'a str,
    pub last_character: &'a str,
}