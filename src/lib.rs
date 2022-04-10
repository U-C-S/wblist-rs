pub mod browser;
mod list;

use browser::Browser;

pub fn get_browser_list() -> Vec<Browser<'static>> {
    let full_list = list::BROWSER_LIST;
    let mut available_browsers: Vec<Browser> = Vec::new();

    for i in full_list {
        let browser_path = i.path;

        use std::path::Path;
        if Path::new(browser_path).exists() {
            available_browsers.push(i);
        }

        /*
        use std::fs;
        let md = fs::metadata(path);
        match md {
            Ok(_) => {
                available_browsers.push(i);
            }
            Err(_) => {
                println!("- {} is not available", i.full_name);
            }
        }
        */
    }

    available_browsers
}

pub fn get_browser_list_from_reg() {
    use registry::Hive::LocalMachine;
    use registry::Security;

    let mut list: Vec<Browser> = Vec::new();
    let key = LocalMachine
        .open("SOFTWARE\\Clients\\StartMenuInternet", Security::Read)
        .unwrap();

    let names = key.keys();
    for name in names {
        let keyname = name.unwrap();
        let lol = keyname.open(Security::Read).unwrap();
        let val = lol.value("");

        match val {
            Ok(x) => {
                println!("{}", x.to_string());
            }
            Err(_) => {
                println!("- Not a browser");
            }
        }
    }
}
