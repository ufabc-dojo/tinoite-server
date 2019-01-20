#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
    pub nick: String,
    pub joined: u32
}
