use std::{fs, path::Path, process::Command};
use crate::{
    parsing::{parse_file, MathFile},
    compiling::{
        compile_directory, add_syndef_to_lib,
        add_axiom_to_lib, add_theo_to_lib,
        verify_theo
    },
    serializing::{read_file, write_lib}
};

fn get_math_file(filepath: &str) -> Result<MathFile, String> {
    let content = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(e) => return Err( format!("{e:?}") )
    };
    let math_file = match parse_file(content) {
        Ok(math_file) => math_file,
        Err(e) => return Err( format!("{e:?}") )
    };
    Ok(math_file)
}

fn try_move_file_to(filepath: &str, dir: String, subdir: &str) -> Result<(), String> {
    let file_name = Path::new(&filepath).file_name()
        .and_then(|s| s.to_str());
    let file_name = match file_name {
        Some(s) => s.to_string(),
        None => return Err( "Could not move file :(".to_string() )
    };
    match fs::rename(filepath, dir + subdir + &file_name) {
        Ok(()) => Ok(()),
        Err(e) => Err( format!("{e:?}") )
    }
}

fn move_entry_to_order_file(filepath: &str, dir: String, subdir: &str) -> Result<(), String> {
    let file_name = Path::new(&filepath).file_name()
        .and_then(|s| s.to_str());
    let file_name = match file_name {
        Some(s) => s.to_string(),
        None => return Err( "Could not open 'order.txt'".to_string() )
    };
    let text = match fs::read_to_string(dir.clone() + r"\order.txt") {
        Ok(text) => text,
        Err(e) => return Err( format!("{e:?}") )
    };
    let mut lines = text.lines();
    let _ = lines.by_ref()
        .take_while(|s| s == &"# Syntax Definitions")
        .collect::<Vec<_>>();
    let syndefs = lines.by_ref()
        .take_while(|s| s == &"# Axioms")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let axioms = lines.by_ref()
        .take_while(|s| s == &"# Theorems")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let theorems = lines
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    
    let line_to_add = subdir.to_owned() + &file_name + "\n";
    let target_section = match subdir {
        r"\syntax_definitions\" => 1,
        r"\axioms\" => 2,
        r"\theorems\" => 3,
        _ => unreachable!()
    };
    let mut file_data = String::from("# Syntax Definition\n");
    for line in syndefs {
        file_data.push_str(line);
        file_data.push('\n');
    };
    if target_section == 1 { file_data.push_str( &line_to_add ) };
    file_data.push_str("\n# Axioms\n");
    for line in axioms {
        file_data.push_str(line);
        file_data.push('\n');
    };
    if target_section == 2 { file_data.push_str( &line_to_add ) };
    file_data.push_str("\n# Theorems\n");
    for line in theorems {
        file_data.push_str(line);
        file_data.push('\n');
    };
    if target_section == 3 { file_data.push_str( &line_to_add ) };
    
    fs::write(dir + r"\order.txt", file_data)
        .map_err(|e| format!("{e:?}"))
}

pub fn compile(dir: String) -> Result<String, String> {
    let lib = match compile_directory(dir.clone()) {
        Ok(lib) => lib,
        Err(e) => return Err( format!("{e:?}") )
    };
    match write_lib(dir + "/library.math", lib) {
        Ok(()) => Ok("Compilation successful!".to_string()),
        Err(e) => return Err( format!("{e:?}") )
    }
}

pub fn add_syndef(dir: String, path: String) -> Result<String, String> {
    let math_file = get_math_file(&path)?;
    let (mut lib, mut references) = match read_file(dir.clone() + "/library.math") {
        Ok((lib, references)) => (lib, references),
        Err(e) => return Err( format!("{e:?}") )
    };
    match add_syndef_to_lib(math_file, &mut lib, &mut references) {
        Ok(()) => (),
        Err(e) => return Err( format!("{e:?}") )
    };
    try_move_file_to(&path, dir.clone(), r"\syntax_definitions\")?;
    move_entry_to_order_file(&path, dir.clone(), r"\syntax_definitions\")?;
    match write_lib(dir + "/library.math", lib) {
        Ok(()) => Ok("Compilation successful!".to_string()),
        Err(e) => return Err( format!("{e:?}") )
    }
}

pub fn add_axiom(dir: String, path: String) -> Result<String, String> {
    let math_file = get_math_file(&path)?;
    let (mut lib, mut references) = match read_file(dir.clone() + "/library.math") {
        Ok((lib, references)) => (lib, references),
        Err(e) => return Err( format!("{e:?}") )
    };
    match add_axiom_to_lib(math_file, &mut lib, &mut references) {
        Ok(()) => (),
        Err(e) => return Err( format!("{e:?}") )
    };
    try_move_file_to(&path, dir.clone(), r"\axioms\")?;
    move_entry_to_order_file(&path, dir.clone(), r"\axioms\")?;
    match write_lib(dir + "/library.math", lib) {
        Ok(()) => Ok("Compilation successful!".to_string()),
        Err(e) => return Err( format!("{e:?}") )
    }
}

pub fn add_theo(dir: String, path: String) -> Result<String, String> {
    let math_file = get_math_file(&path)?;
    let (mut lib, mut references) = match read_file(dir.clone() + "/library.math") {
        Ok((lib, references)) => (lib, references),
        Err(e) => return Err( format!("{e:?}") )
    };
    match add_theo_to_lib(math_file, &mut lib, &mut references) {
        Ok(()) => (),
        Err(e) => return Err( format!("{e:?}") )
    };
    try_move_file_to(&path, dir.clone(), r"\theorems\")?;
    move_entry_to_order_file(&path, dir.clone(), r"\theorems\")?;
    match write_lib(dir + "/library.math", lib) {
        Ok(()) => Ok("Compilation successful!".to_string()),
        Err(e) => return Err( format!("{e:?}") )
    }
}

pub fn verify(dir: String, path: String) -> Result<String, String> {
    let math_file = get_math_file(&path)?;
    let (mut lib, mut references) = match read_file(dir.clone() + "/library.math") {
        Ok((lib, references)) => (lib, references),
        Err(e) => return Err( format!("{e:?}") )
    };
    match verify_theo(math_file, &mut lib, &mut references) {
        Ok(()) => Ok("Theorem is valid".to_string()),
        Err(e) => return Err( format!("{e:?}") )
    }
}

pub fn open_editor(dir: String) -> Result<String, String> {
    Command::new("wezterm")
        .arg("start")
        .args(["--cwd", dir.as_str()])
        .args(["--", "target/debug/mled.exe"])
        .status()
        .map(|_| "".to_string())
        .map_err(|e| e.to_string())
}
