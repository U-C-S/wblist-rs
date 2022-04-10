use crate::browser::Browser;

pub static BROWSER_LIST: [Browser<&str>; 5] = [
    Browser {
        full_name: "Google Chrome",
        short_name: "Chrome",
        path: "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    },
    Browser {
        full_name: "Mozilla Firefox",
        short_name: "Firefox",
        path: "C:\\Program Files\\Mozilla Firefox\\firefox.exe",
    },
    Browser {
        full_name: "Microsoft Edge",
        short_name: "Edge",
        path: "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
    },
    Browser {
        full_name: "Opera",
        short_name: "Opera",
        path: "C:\\Program Files\\Opera\\launcher.exe",
    },
    Browser {
        full_name: "Brave",
        short_name: "Brave",
        path: "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
    },
];
