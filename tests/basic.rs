use wblist::{from_fs_search, from_reg};

#[test]
fn list_the_browsers() {
    let x = from_fs_search();
    assert_eq!(x.len(), 3usize);
}

#[test]
fn list_the_browsers_from_reg() {
    let x = from_reg();
    for _i in &x {
        // println!("{:?}", i);
    }
}
