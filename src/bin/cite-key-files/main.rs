use biblatex::Bibliography;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bibliography_path_arg = args.get(1).unwrap();

    let bibliography_path = Path::new(bibliography_path_arg);
    let bibliography_source = fs::read_to_string(bibliography_path).unwrap();
    let bibliography = Bibliography::parse(bibliography_source.as_str()).unwrap();

    let folder = bibliography_path.parent().unwrap();
    let file_names: Vec<String> = folder
        .read_dir().unwrap()
        .map(|file| file.unwrap().file_name())
        .map(|file_name| file_name.into_string().unwrap())
        .collect();

    for entry in bibliography {
        println!("Checking {}", entry.key);
        let file = file_names.iter()
            .find(|file_name| file_name.starts_with(&entry.key)).unwrap();
        println!("Found {}", file)
    }
}
