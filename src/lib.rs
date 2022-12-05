extern crate core;

mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn get_data_as_string(is_example:bool, day: &str) -> String {
    let mut pathbuf:PathBuf = env::current_dir().unwrap();
    pathbuf.push("src");
    pathbuf.push(day);
    pathbuf.push("test_data");
    if is_example {
        pathbuf.push("input_data_example")
    }
    else {
        pathbuf.push("input_data_actual")
    };
    let mut file = match File::open(&pathbuf) {
        Err(why) => panic!("Wrong pathbuf! {}", why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file {}", why),
        Ok(_) => {
            s
        }
    }
}