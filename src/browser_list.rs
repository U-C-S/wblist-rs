use crate::browser::Browser;

fn list_of_browser_str() -> &'static str {
    include_str!("..\\resources\\wblist.json")
}

pub fn list_of_browsers() -> Vec<Browser<String>> {
    let list_of_browser: Vec<Browser<String>> =
        serde_json::from_str(list_of_browser_str()).expect("wblist.json parse error");
    list_of_browser
}

#[cfg(test)]
mod tests {
    use super::list_of_browsers;

    #[test]
    fn it_works() {
        let x = list_of_browsers();
        println!("The number of Browsers in the browser db is \"{}\"", &x.len());
    }
}
