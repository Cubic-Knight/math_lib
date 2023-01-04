use crate::settings::{Settings, save_settings_to_file};

fn settings_intro(settings: &Settings) -> String {
    format!(
        "\n\
        color: {}\n\
        lib_path: '{}'\n\
        safe: {}\n\
        ", settings.color, settings.lib_path, settings.safe
    ) 
}

fn flag_description(settings: &Settings, flag_name: &String) -> Option<String> {
    let (value, description) = match flag_name.as_str() {
        "color" => (settings.color.to_string(), "NOT IMPLEMENTED"),
        "lib_path" => (
            settings.lib_path.clone(), "The path to the library directory"
        ),
        "safe" => (settings.safe.to_string(), "Whether safe mode is activated"),
        _ => return None
    };
    Some(
        format!("\nFlag {flag_name}: {value}\n  {description}")
    )
}

fn set_flag(settings: &mut Settings, flag_name: &String, value: String) -> Result<String, String> {
    let operation_result = match flag_name.as_str() {
        "color" => match value.as_str() {
            "true" => {
                settings.color = true;
                save_settings_to_file(settings)
            },
            "false" => {
                settings.color = false;
                save_settings_to_file(settings)
            },
            _ => return Err(
                format!("'color' needs a boolean value, found '{value}'")
            )
        },
        "safe" => match value.as_str() {
            "true" => {
                settings.safe = true;
                save_settings_to_file(settings)
            },
            "false" => {
                settings.safe = false;
                save_settings_to_file(settings)
            },
            _ => return Err(
                format!("'safe' needs a boolean value, found '{value}'")
            )
        },
        "lib_path" => {
            settings.lib_path = value.clone();
            save_settings_to_file(settings)
        },
        _ => return Err( format!("Unknown flag '{flag_name}'") )
    };
    match operation_result {
        Ok(()) => Ok( format!("Successfully set '{flag_name}' to '{value}'") ),
        Err(e) => Err( format!("{e:?}") )
    }
}

pub fn handle_flag_command(
    name: Option<String>, value: Option<String>, settings: &mut Settings
) -> Result<String, String> {
    match (name, value) {
        (None, _) => Ok( settings_intro(settings) ),
        (Some(name), None) => match flag_description(settings, &name) {
            Some(text) => Ok(text),
            None => Err( format!("Unknown flag '{name}'") )
        },
        (Some(name), Some(value)) => set_flag(settings, &name, value)
    }
}