pub mod browser;
pub mod browser_list;

mod wblist;

pub fn from_fs_search() -> Vec<browser::Browser> {
    wblist::from_fs_search()
}

pub fn from_reg() -> Vec<browser::Browser> {
    wblist::from_reg()
}
