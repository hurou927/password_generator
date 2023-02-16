use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Password {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    version: String,
    passwords: Vec<Password>,
}

impl Output {
    pub fn new(passwords: Vec<String>) -> Self {
        Output {
            version: "1.0.0".to_owned(),
            passwords: passwords
                .iter()
                .map(|x| Password {
                    text: x.to_owned(),
                })
                .collect(),
        }
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|er| format!("json error: {er:?}"))
    }
}
