pub enum Type {
    Echo,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Msg {
    src: String,
    dest: String,
    body: Vec<u8>,
}
