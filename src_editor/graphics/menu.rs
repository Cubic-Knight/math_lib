use std::{fs, path::Path};

pub struct MenuGraphics {
    pub cursor: usize,
    pub lines: Vec<MenuLine>
}

pub enum MenuLine {
    RootDirectory(String),
    // SubDirectory(name, is_last_dir)
    SubDirectory(String, bool),
    // File(name, path, is_in_last_dir, is_last_in_dir)
    File(String, String, bool, bool)
}

pub fn get_menu() -> Result<MenuGraphics, ()> {
    let contents = fs::read_to_string("order.txt").map_err(|_| ())?;

    let mut result_lines = Vec::new();

    let current_dir = std::env::current_dir().map_err(|_| ())?;
    let cwd = current_dir.to_str().ok_or(())?.to_string();
    result_lines.push(MenuLine::RootDirectory(cwd));
    // get registered files
    for line in contents.lines() {
        let menu_line = match line {
            "" => continue,
            "# Syntax Definitions" => {
                MenuLine::SubDirectory("syntax_defintions".to_string(), false)
            },
            "# Axioms" => {
                // If the last object is a file, encode the fact it is last
                let mut last_item = result_lines.pop().ok_or(())?;
                if let MenuLine::File(name, path, _, _) = last_item {
                    last_item = MenuLine::File(name, path, false, true);
                };
                result_lines.push(last_item);

                MenuLine::SubDirectory("axioms".to_string(), false)
            },
            "# Theorems" => {
                // If the last object is a file, encode the fact it is last
                let mut last_item = result_lines.pop().ok_or(())?;
                if let MenuLine::File(name, path, _, _) = last_item {
                    last_item = MenuLine::File(name, path, false, true);
                };
                result_lines.push(last_item);

                MenuLine::SubDirectory("theorems".to_string(), false)
            },
            other => {
                let file_name = Path::new(&other).file_name()
                    .and_then(|s| s.to_str())
                    .ok_or(())?
                    .to_string();
                let path = other.chars().skip(1).collect::<String>();
                MenuLine::File(file_name, path, false, false)
            }
        };
        result_lines.push(menu_line);
    };
    // If the last object is a file, encode the fact it is last
    let mut last_item = result_lines.pop().ok_or(())?;
    if let MenuLine::File(name, path, _, _) = last_item {
        last_item = MenuLine::File(name, path, false, true);
    };
    result_lines.push(last_item);

    // get files in pending
    result_lines.push(MenuLine::SubDirectory("pending".to_string(), true));
    let mut pending_files = fs::read_dir("pending")
        .map_err(|_| ())?
        .map(|file|
            file.map_err(|_| ())?
                .file_name()
                .to_str()
                .map(|s| s.to_string())
                .ok_or(())
        )
        .collect::<Result<Vec<_>, _>>()?;
    pending_files.sort();
    for file_name in pending_files {
        let path = r"pending\".to_string() + file_name.as_str();
        result_lines.push(MenuLine::File(file_name, path, true, false))
    };

    // If the last object is a file, encode the fact it is last
    let mut last_item = result_lines.pop().ok_or(())?;
    if let MenuLine::File(name, path, _, _) = last_item {
        last_item = MenuLine::File(name, path, true, true);
    };
    result_lines.push(last_item);

    Ok(
        MenuGraphics {
            cursor: 2,
            lines: result_lines
        }
    )
}
