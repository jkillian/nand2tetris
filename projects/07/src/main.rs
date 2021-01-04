use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use vmtranslator::parser;
use vmtranslator::writer;

struct File {
    content: String,
    filename: String,
}

fn get_files(path: Option<&str>) -> Result<Vec<File>, Box<dyn std::error::Error>> {
    match path {
        None | Some("-") => {
            let mut data = String::new();
            io::stdin().read_to_string(&mut data)?;
            Ok(vec![File {
                content: data,
                filename: "stdin".into(),
            }])
        }
        Some(file_path) if file_path.ends_with(".vm") => Ok(vec![File {
            content: fs::read_to_string(file_path)?,
            filename: Path::new(file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
        }]),
        Some(dir_path) => {
            let results = fs::read_dir(dir_path)?
                .filter_map(|entry| {
                    let file_path = match entry {
                        Err(_) => return None,
                        Ok(entry) => entry.path(),
                    };
                    match file_path.extension() {
                        Some(ext) if ext == "vm" => Some(File {
                            content: fs::read_to_string(&file_path).unwrap(),
                            filename: file_path.file_name().unwrap().to_str().unwrap().into(),
                        }),
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
        let commands = parser::parse_file_contents(&file.content[..])?;

        writer.set_filename(file.filename);
        for cmd in commands {
            println!("{}", writer.write_command(&cmd));
        }
    }

    Ok(())
}
