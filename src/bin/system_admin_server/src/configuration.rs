#[derive(Serialize, Deserialize)]
pub struct Configuration {
    port : i32,
    plain_text : bool,
}

impl std::default::Default for Configuration {
    fn default() -> Self { Self{ port : 0, plain_text : false}}
}