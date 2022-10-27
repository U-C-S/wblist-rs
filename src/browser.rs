use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Browser {
    pub full_name: String,
    pub short_name: Option<String>,

    /// The path to the browser's executable.
    pub path: String,

    /// Type of Browser
    // pub btype: BrowserType,

    /// Icons from https://github.com/alrra/browser-logos/tree/main/src
    ///
    /// We just use the name of the foider
    pub icon: String,

    pub data_dir: Option<String>,
}

impl Browser {
    pub fn new(full_name: String, path: String, short_name: Option<String>) -> Self {
        Self {
            full_name,
            path,
            // btype: BrowserType::Unknown,
            short_name,
            icon: "".to_string(),
            data_dir: None,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub enum BrowserType {
    Chromium,
    Firefox,

    #[default]
    Unknown,
}

// Google Chrome
// Mozilla Firefox
// Microsoft Edge
// Opera
// Vivaldi
// Brave
