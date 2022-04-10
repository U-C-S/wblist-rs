use wblist::{get_browser_list, get_browser_list_from_reg};

// #[test]
// fn it_works() {
//     assert_eq!(BROWSER_LIST[0].short_name, "Chrome");
// }

#[test]
fn list_the_browsers() {
    let x = get_browser_list();
    assert_eq!(x.len(), 2usize);
}

#[test]
fn list_the_browsers_from_reg() {
    get_browser_list_from_reg();
}
