use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .last()
        .ok_or("Unable to resolve path")
        .unwrap()
        .to_string();
    traverse(path, 0);
}

fn traverse(dir: String, spaces: usize) {
    let paths = fs::read_dir(dir.clone());
    let padding = str::repeat(" ", spaces);
    match paths {
        Ok(path) => {
            path.for_each(|f: Result<fs::DirEntry, std::io::Error>| {
                let is_dir = f.as_ref().unwrap().file_type().unwrap().is_dir();
                let file_or_dir_name = f.unwrap().file_name().into_string().unwrap();
                println!("{padding}{file_or_dir_name}");
                if is_dir {
                    let nested_path = format!("{dir}/{file_or_dir_name}");
                    traverse(nested_path, spaces + 4);
                }
            });
        }
        Err(err) => println!("{err}"),
    }
}
