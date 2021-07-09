use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

fn get_history_file_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("couldn't interpret $HOME");
    let history_file_path = Path::new(&home_dir).join(".local/share/fish/fish_history");
    history_file_path
}

fn main() -> io::Result<()> {
    let history_file_path = get_history_file_path();

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
fn it_works() {}
