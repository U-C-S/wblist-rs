#[derive(Debug)]
pub struct Browser<'a> {
    pub full_name: &'a str,
    pub short_name: &'a str,
    pub path: &'a str,
}

impl<'a> Browser<'a> {
    pub fn new(full_name: &'a str, short_name: &'a str, path: &'a str) -> Self {
        Self {
            full_name,
            short_name,
            path,
        }
    }
}
