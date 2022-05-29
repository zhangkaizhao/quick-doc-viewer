use tide::prelude::*;

#[derive(Deserialize)]
#[serde(default)]
pub struct Page {
    pub raw: bool,
    pub format: String,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            raw: false,
            format: "".to_string(),
        }
    }
}
