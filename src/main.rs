extern crate yaml_rust;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
};
use yaml_rust::yaml;

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

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("    ");
    }
}

fn dump_node(doc: &yaml::Yaml, indent: usize) {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            for x in v {
                dump_node(x, indent + 1);
            }
        }
        yaml::Yaml::Hash(ref h) => {
            for (k, v) in h {
                //print_indent(indent);
                if k.as_str().unwrap() == "cmd" {
                    dump_node(v, indent + 1);
                }
            }
        }
        _ => {
            //print_indent(indent);
            println!("{:?}", doc);
        }
    }
}

fn main() -> io::Result<()> {
    let history_file_path = get_history_file_path(Env::Dev);

    let file = File::open(history_file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    reader.read_to_string(&mut buf).unwrap();

    let histories = yaml::YamlLoader::load_from_str(&buf).unwrap();
    let history_yaml = &histories[0];

    println!("---");
    dump_node(&history_yaml, 0);

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
