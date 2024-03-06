extern crate dirs;
use std::{env, fs};

const RES_DIR_LINUX: &str = r"/.minecraft/resourcepacks/";
const RES_DIR_WINDOWS: &str = r"\AppData\Roaming\.minecraft\resourcepacks\";
const TARGET_PATH: &str = "./packs/";

fn main() {
    let mut home_path = String::new();
    match dirs::home_dir() {
        Some(dir) => home_path = dir.display().to_string(),
        None => println!("could not find home dir!")
    }
    let mut packs_path = String::new();
    match env::consts::OS {
        "linux" => packs_path = format!("{}{}", home_path, RES_DIR_LINUX),
        "windows" => packs_path = format!("{}{}", home_path, RES_DIR_WINDOWS),
        _ => println!("{} not supported", env::consts::OS)
    }
    let paths = fs::read_dir(packs_path).unwrap();
    
    for path in paths {
        let path = path.unwrap().path().display().to_string();
        let current_path = std::path::Path::new(&path);
        if !path.ends_with("zip") {continue};
        
        let current_pack = fs::File::open(current_path).unwrap();
        let mut pack_name: &str = <&str>::try_from(current_path.file_name().unwrap()).unwrap();
        println!("extracting {} to /packs/", pack_name);
        pack_name = pack_name.strip_suffix(".zip").unwrap();
        
        let current_target_path = format!("{}{}/",TARGET_PATH,pack_name);
        let mut archive = zip::ZipArchive::new(current_pack).unwrap();
        let _ = archive.extract(&current_target_path);
    }
}