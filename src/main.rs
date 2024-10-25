use rand::Rng;
use std::{env, fs};

struct Settings {
    key_length: usize,
    char_set: String,
    output_file: String,
}

fn load_settings() -> Settings {
    let mut s = Settings { // default settings
        key_length: 512,
        char_set: {
            let mut s = String::new();
            for c in 0x21..0x7F {
                s.push(c as u8 as char);
            }
            String::from(s)
        },
        output_file: "key.txt".to_string(),
    };

    let args: Vec<String> = env::args().collect();
    let flags = vec!["-l", "-c", "-o"];
    let mut flag = "--";

    for arg in args.iter() {
        if flag != "" {
            match flag {
                "-l" => {
                    s.key_length = arg.parse::<usize>().unwrap();
                }
                "-c" => {
                    s.char_set = arg.to_string();
                }
                "-o" => {
                    s.output_file = arg.to_string();
                }
                "--" => {
                    // ignore the first argument
                }
                _ => {
                    println!("Unknown argument: {}", arg);
                }
            }
            flag = "";
        } else {
            if flags.contains(&arg.as_str()) {
                flag = arg.as_str();
            } else {
                println!("Unknown argument: {}", arg);
            }
        }
    }

    Settings {
        key_length: s.key_length,
        char_set: s.char_set,
        output_file: s.output_file,
    }
}

fn gen_random_key(key_length: usize, char_set: String) -> String {
    let mut rng = rand::thread_rng();
    let mut key = String::with_capacity(key_length);
    for _ in 0..key_length {
        let i = rng.gen_range(0..char_set.len());
        key.push(char_set.chars().nth(i).unwrap());
    }

    String::from(key)
}

fn main() {
    let settings = load_settings();
    let key = gen_random_key(settings.key_length, settings.char_set);
    fs::write(&settings.output_file, key).expect("Unable to write file");
    println!("Key written to {}", settings.output_file);
}
