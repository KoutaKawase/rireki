use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
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

fn main() -> io::Result<()> {
    let history_file_path = get_history_file_path(Env::Dev);

    let file = File::open(history_file_path).unwrap();
    let mut reader = BufReader::new(file);

    for _ in 0..5 {
        let mut buf = String::new();
        const EOF: usize = 0;
        match reader.read_line(&mut buf) {
            Ok(EOF) => {
                println!("END");
                break;
            }
            Ok(_) => (),
            Err(why) => panic!("Something wrong!! {}", why.to_string()),
        }
        println!("{}", buf);
    }

    Ok(())
}

#[test]
fn test_get_history_file_path() {
    let home_dir = env::var("HOME").expect("couldn't interpret $HOME");
    let dev_path = PathBuf::from("fixtures/sample_fish_history.txt");
    let prod_path = Path::new(&home_dir).join(".local/share/fish/fish_history");
    assert_eq!(get_history_file_path(Env::Dev), dev_path);
    assert_eq!(get_history_file_path(Env::Production), prod_path);
}
