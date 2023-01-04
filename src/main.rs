mod parsing;
mod compiling;
mod serializing;

mod commands;
use commands::{
    compile, add_syndef,
    add_axiom, add_theo,
    verify, open_editor
};

mod flags;
use flags::handle_flag_command;

mod settings;
use settings::get_settings;

use macro_clap::*;
cli!(
    const ARG_PARSER: ArgParser<"This is math_lib"> = [
        branch!(command as Command {
            "compile" |> Compile => {},
            "add_sd" |> AddSyndef => {
                arg!(path as String)
            },
            "add_ax" |> AddAxiom => {
                arg!(path as String)
            },
            "add" |> AddTheo => {
                arg!(path as String)
            },
            "verify" |> Verify => {
                arg!(path as String)
            },
            "edit" |> Edit => {},
            "flag" |> Flag => {
                maybe!(flag_name as (Option<String>)),
                maybe!(flag_value as (Option<String>))
            }
        })
    ]
);

fn main() {
    let command = match ARG_PARSER.parse_args() {
        Ok(command) => command,
        Err(message) => {
            println!("{message}");
            return;
        }
    };
    let mut settings = match get_settings() {
        Ok(settings) => settings,
        Err(e) => {
            println!("ERROR: {e:?}");
            return;
        }
    };
    let dir = settings.lib_path.clone();
    let command_result = match command {
        Command::Compile() => compile(dir),
        Command::AddSyndef(path) => add_syndef(dir, path),
        Command::AddAxiom(path) => add_axiom(dir, path),
        Command::AddTheo(path) => add_theo(dir, path),
        Command::Verify(path) => verify(dir, path),
        Command::Edit() => open_editor(dir),
        Command::Flag(name, value) => {
            handle_flag_command(name, value, &mut settings)
        }
    };
    match command_result {
        Ok(message) => println!("SUCCESS: {message}"),
        Err(message) => println!("ERROR: {message}")
    }
}
