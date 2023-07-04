use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .last()
        .ok_or("Unable to resolve path")
        .unwrap()
        .to_string();
    let last_in_initial_path = path.split('/').last().unwrap();
    let result = traverse(&path);
    result.into_iter().for_each(|f| {
        let file_path_from_root = format!("{last_in_initial_path}/{f}");
        println!("{file_path_from_root}");
    });
}

fn traverse(dir: &String) -> Vec<String> {
    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir.clone());
    let directory_entries: fs::ReadDir = read_dir_result.unwrap();
    return directory_entries
        .flat_map(|f: Result<fs::DirEntry, std::io::Error>| {
            let is_dir: bool = f.as_ref().unwrap().file_type().unwrap().is_dir();
            let file_or_dir_name: String = f.unwrap().file_name().into_string().unwrap();
            if is_dir {
                let nested_path: String = format!("{dir}/{file_or_dir_name}");
                return traverse(&nested_path)
                    .into_iter()
                    .map(|f| format!("{file_or_dir_name}/{f}"))
                    .collect();
            } else {
                return vec![format!("{file_or_dir_name}")];
            }
        })
        .collect();
}
