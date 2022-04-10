use wblist::get_browser_list::{from_fs_search, from_reg};
use wblist::list::BROWSER_LIST;

#[test]
fn it_works() {
    assert_eq!(BROWSER_LIST[0].short_name, "Chrome");
}

#[test]
fn list_the_browsers() {
    let x = from_fs_search();
    assert_eq!(x.len(), 2usize);
}

#[test]
fn list_the_browsers_from_reg() {
    assert_eq!(from_reg().len(), 3usize);
}
