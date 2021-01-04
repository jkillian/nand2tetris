use std::env;
use std::fs;
use std::io::{self, Read};

use vmtranslator::parser;
use vmtranslator::writer;

fn get_files(path: Option<&str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match path {
        None | Some("-") => {
            let mut data = String::new();
            io::stdin().read_to_string(&mut data)?;
            Ok(vec![data])
        }
        Some(file_path) if file_path.ends_with(".vm") => Ok(vec![fs::read_to_string(file_path)?]),
        Some(dir_path) => {
            let results = fs::read_dir(dir_path)?
                .filter_map(|entry| {
                    let file_path = match entry {
                        Err(_) => return None,
                        Ok(entry) => entry.path(),
                    };
                    match file_path.extension() {
                        Some(ext) if ext == "vm" => fs::read_to_string(file_path).ok(),
                        _ => None,
                    }
                })
                .collect();
            Ok(results)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        panic!("Usage: {} [filename or directory]", args[0]);
    }

    let files = get_files(args.get(1).map(|s| &s[..]))?;
    let mut writer = writer::CodeWriter::new();
    for file in files {
        let commands = parser::parse_file_contents(&file[..])?;
        for cmd in commands {
            println!("{}", writer.write_command(&cmd));
        }
    }

    Ok(())
}
