use crate::browser::Browser;
use crate::browser_list::list_of_browsers;

pub fn from_fs_search() -> Vec<Browser> {
    let mut available_browsers: Vec<Browser> = Vec::new();
    let browser_list = list_of_browsers();

    for i in browser_list {
        let browser_path = &i.path;

        use std::path::Path;
        if Path::new(&browser_path).exists() {
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

pub fn from_reg() -> Vec<Browser> {
    use registry::Hive::LocalMachine;
    use registry::Security;

    let mut list: Vec<Browser> = Vec::new();
    let key = LocalMachine
        .open("SOFTWARE\\Clients\\StartMenuInternet", Security::Read)
        .unwrap();

    let names = key.keys();
    for name in names {
        let keyname = name.unwrap();

        let browserkey = keyname.open(Security::Read).unwrap();
        let name = browserkey.value("").unwrap_or(registry::Data::None);
        // println!("{}", &name);

        let pathkey = browserkey.open("shell\\open\\command", Security::Read);

        if let Ok(path) = pathkey {
            let path = path.value("").unwrap();
            let b = Browser::new(name.to_string(), path.to_string(), None);
            list.push(b);
        }
    }

    list
}
