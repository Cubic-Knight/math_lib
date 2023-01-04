use std::{fs, io};

pub struct Settings {
    pub color: bool,
    pub lib_path: String,
    pub safe: bool
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            color: true,
            lib_path: "".to_string(),
            safe: true
        }
    }
}

#[derive(Debug)]
pub enum SettingsError {
    IOError(io::Error),
    UnparsableAsBool(usize, String, String),
    UnparsableAsString(usize, String, String),
    UnknownOption(usize, String),
}

fn parse_as_bool(name: &str, value: &str, index: usize) -> Result<bool, SettingsError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(
            SettingsError::UnparsableAsBool(index, name.to_string(), value.to_string())
        )
    }
}

fn parse_as_string(name: &str, value: &str, index: usize) -> Result<String, SettingsError> {
    if !value.starts_with('"') || !value.ends_with('"') {
        return Err(
            SettingsError::UnparsableAsString(index, name.to_string(), value.to_string())
        );
    };
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    Ok(chars.collect())
}

pub fn get_settings() -> Result<Settings, SettingsError> {
    let contents = match fs::read_to_string("settings.txt") {
        Ok(contents) => contents,
        Err(e) => return Err(SettingsError::IOError(e))
    };
    let mut res = Settings::default();
    for (i, line) in contents.lines().enumerate() {
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        match name.trim() {
            "COLOR" => {
                let value = parse_as_bool(name, value, i)?;
                res.color = value;
            },
            "LIB_PATH" => {
                let value = parse_as_string(name, value, i)?;
                res.lib_path = value;
            },
            "SAFE" => {
                let value = parse_as_bool(name, value, i)?;
                res.safe = value;
            },
            other => return Err(
                SettingsError::UnknownOption(i, other.to_string())
            )
        };
    };
    Ok(res)
}

pub fn save_settings_to_file(settings: &Settings) -> io::Result<()> {
    let contents = format!(
        "\
        COLOR={}\n\
        LIB_PATH=\"{}\"\n\
        SAFE={}\n",
        settings.color, settings.lib_path, settings.safe
    );
    fs::write("settings.txt", contents)
}
