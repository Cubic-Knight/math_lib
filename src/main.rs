#[allow(unused)]
use std::fs;

mod parsing;
#[allow(unused)]
use parsing::parse_file;

mod compiling;
#[allow(unused)]
use compiling::compile;

mod serializing;

fn main() {
    /*
    let content = fs::read_to_string("library/theorems/a").unwrap();
    let math_file = parse_file(content);
    match math_file {
        Ok(math_file) => println!("{:?}", math_file),
        Err(e) => println!("{:?}", e)
    }
     */
    match compile("library".to_string()) {
        Ok(()) => println!("Compilation successful"),
        Err(e) => println!("{e:?}")
    };
}
