use crate::statics::{UZ_CYR_AFF, UZ_CYR_DIC, UZ_LAT_AFF, UZ_LAT_DIC};
use std::path::Path;

#[cfg(target_os = "linux")]
const LOCATION: &str = "/usr/share/hunspell";

#[cfg(target_os = "macos")]
const LOCATION: &str = "~/Library/Spelling";

#[cfg(target_os = "windows")]
const LOCATION: &str = "/usr/share/hunspell";

pub fn bootstrap() {
    let files: Vec<(&str, &str)> = vec![
        ("uz-cyr.aff", UZ_CYR_AFF),
        ("uz-lat.aff", UZ_LAT_AFF),
        ("uz-cyr.dic", UZ_CYR_DIC),
        ("uz-lat.dic", UZ_LAT_DIC),
    ];

    if !Path::new("./dictionary").exists() {
        std::fs::create_dir("dictionary").expect("Couldn't create dictionary directory");
    } else {
        println!("Seems like dictionary directory already exists");
    }

    for file in files {
        let link = format!("./dictionary/{}", file.0);
        let path = Path::new(link.as_str());

        if !path.exists() {
            std::fs::write(link.as_str(), file.1)
                .unwrap_or_else(|_| panic!("Unable to write file at {}", link));
        } else {
            println!("The directory exists!");
        }
    }
    println!("Assets has been updated successfully!");

    println!("Dictionaries have been successfully generated!");
    println!("Make sure to move generated contents to: {}", LOCATION);
    println!("You can do that by: cp ./dictionary/* {}", LOCATION);
}
