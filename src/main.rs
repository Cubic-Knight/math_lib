#[allow(unused)]
use std::fs;

mod parsing;
#[allow(unused)]
use parsing::parse_file;

mod compiling;
#[allow(unused)]
use compiling::compile;

mod serializing;
use serializing::read_file;

fn main() {
    let dir = "library".to_string();
    match compile(dir.clone()) {
        Ok(()) => println!("Compilation successful"),
        Err(e) => println!("{e:?}")
    };
    match read_file(dir + "/library.math") {
        Ok(lib) => println!("{lib:#?}"),
        Err(e) => println!("{e:?}")
    }
}
