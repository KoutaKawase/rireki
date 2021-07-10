use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

pub enum Env {
    Production,
    Dev,
}

fn get_history_file_path(env: Env) -> PathBuf {
    let home_dir = env::var("HOME").expect("couldn't interpret $HOME");
    let path = match env {
        Env::Dev => PathBuf::from("fixtures/sample_fish_history.txt"),
        Env::Production => Path::new(&home_dir).join(".local/share/fish/fish_history"),
    };
    path
}

fn main() {
    let history_file_path = get_history_file_path(Env::Dev);

    let file = File::open(history_file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    reader.read_to_string(&mut buf).unwrap();
}

#[test]
fn test_get_history_file_path() {
    let home_dir = env::var("HOME").expect("couldn't interpret $HOME");
    let dev_path = PathBuf::from("fixtures/sample_fish_history.txt");
    let prod_path = Path::new(&home_dir).join(".local/share/fish/fish_history");
    assert_eq!(get_history_file_path(Env::Dev), dev_path);
    assert_eq!(get_history_file_path(Env::Production), prod_path);
}
