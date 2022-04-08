pub mod browser;
mod list;

pub fn get_browser_list() {
    let x = list::BROWSER_LIST;
    for i in x {
        println!("{}", i.name);
    }
}

#[cfg(test)]
mod tests {
    use crate::list;

    #[test]
    fn it_works() {
        let result = list::BROWSER_LIST;
        assert_eq!(result[0].name, "Chrome");
    }
}
