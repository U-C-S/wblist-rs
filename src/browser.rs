use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Browser {
    pub full_name: String,
    pub short_name: String,
    pub path: String,
}

impl Browser {
    pub fn new(full_name: String, short_name: String, path: String) -> Self {
        Self {
            full_name,
            short_name,
            path,
        }
    }
}
