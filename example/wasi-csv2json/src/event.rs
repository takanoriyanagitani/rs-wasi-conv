#[derive(serde::Deserialize, serde::Serialize)]
pub struct PartialEvent {
    id: String,
    datetime: String,
    event: String,
    key: String,
    val: String,
}
impl PartialEvent {
    pub fn key_eq(&self, other: &str) -> bool {
        self.key.eq(other)
    }
}
