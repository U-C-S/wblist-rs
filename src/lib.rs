pub mod browser;
mod list;

use browser::Browser;
use std::fs;

pub fn get_browser_list() -> Vec<Browser<'static>> {
    let full_list = list::BROWSER_LIST;
    let mut available_browsers: Vec<Browser> = Vec::new();

    for i in full_list {
        let path = i.path;
        let md = fs::metadata(path);
        match md {
            Ok(_) => {
                available_browsers.push(i);
            }
            Err(_) => {
                println!("- {} is not available", i.full_name);
            }
        }
    }

    available_browsers
}

#[cfg(test)]
mod tests {
    use crate::{get_browser_list, list::BROWSER_LIST};

    #[test]
    fn it_works() {
        assert_eq!(BROWSER_LIST[0].short_name, "Chrome");
    }

    #[test]
    fn list_the_browsers() {
        let x = get_browser_list();
        assert_eq!(x.len(), 2usize);
    }
}
