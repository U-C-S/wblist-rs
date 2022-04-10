pub mod browser;
pub mod list;

mod wblist;
mod browser_list;

pub fn from_fs_search() -> Vec<browser::Browser<&'static str>> {
    wblist::from_fs_search()
}

pub fn from_reg() -> Vec<browser::Browser<String>> {
    wblist::from_reg()
}
