pub mod browser;
pub mod list;

pub mod get_browser_list {
    use crate::browser::Browser;
    use crate::list::BROWSER_LIST;

    pub fn from_fs_search() -> Vec<Browser<'static>> {
        let mut available_browsers: Vec<Browser> = Vec::new();

        for i in BROWSER_LIST {
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

    pub fn from_reg() -> Vec<Browser<'static>> {
        use registry::Hive::LocalMachine;
        use registry::Security;

        let mut list: Vec<Browser<'static>> = Vec::new();
        let key = LocalMachine
            .open("SOFTWARE\\Clients\\StartMenuInternet", Security::Read)
            .unwrap();

        let names = key.keys();
        for name in names {
            let keyname = name.unwrap();

            let browserkey = keyname.open(Security::Read).unwrap();
            let name = browserkey.value("");

            let pathkey = browserkey.open("shell\\open\\command", Security::Read);
            match pathkey {
                Ok(path) => {
                    let path = path.value("");
                    list.push(Browser {
                        full_name: &name.unwrap().to_string(),
                        short_name: "",
                        path: &path.unwrap().to_string(),
                    });
                }
                Err(_) => {}
            };
        }

        list
    }
}
