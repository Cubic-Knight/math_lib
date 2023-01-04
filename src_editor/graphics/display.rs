use std::io::{stdout, Write};
use super::{
    MenuGraphics, MenuLine,
    FileGraphics
};
use crate::parsing::{
    FileLine, ProvenState,
    ColoredString, Color
};

const LINE_TERMINATOR: &'static str = "\x1b[m\n\r";

pub fn display_menu(menu: &MenuGraphics) {
    let MenuGraphics { cursor, lines } = menu;

    print!("\x1b[2J\x1b[H"); // Clear display
    for (index, line) in lines.into_iter().enumerate() {
        let (indent, color, name) = match line {
            MenuLine::RootDirectory(name) => {
                ("", "\x1b[1;4;33m", name)
            },
            MenuLine::SubDirectory(name, is_last) => {
                let indent = match is_last {
                    true => "\x1b[1;33m┖──\x1b[m ",
                    false => "\x1b[1;33m┠──\x1b[m "
                };
                let color = match index + 1 == *cursor {
                    true => "\x1b[1;7;34m",
                    false => "\x1b[1;34m"
                };
                (indent, color, name)
            },
            MenuLine::File(name, _, is_in_last_dir, is_last_in_dir) => {
                let indent = match (is_in_last_dir, is_last_in_dir) {
                    (true, true) => "    \x1b[1;34m┖──\x1b[m ",
                    (true, false) => "    \x1b[1;34m┠──\x1b[m ",
                    (false, true) => "\x1b[1;33m┃\x1b[m   \x1b[1;34m┖──\x1b[m ",
                    (false, false) => "\x1b[1;33m┃\x1b[m   \x1b[1;34m┠──\x1b[m ",
                };
                let color = match index + 1 == *cursor {
                    true => "\x1b[7m",
                    false => "\x1b[m"
                };
                (indent, color, name)
            },
        };
        print!("{}{}{}{}", indent, color, name, LINE_TERMINATOR);
    };
    stdout().flush().unwrap();
}


fn colored_string_display(string: &ColoredString) -> String {
    let ColoredString { characters, colors } = string;
    let mut result_string = String::new();
    let mut current_color = Color::Normal;
    for (chr, col) in characters.into_iter().zip(colors) {
        if *col != current_color {
            result_string.push_str( &format!("\x1b[{}m", *col as u8) );
            current_color = *col;
        };
        result_string.push(*chr);
    };
    result_string
}

pub fn display_file(file: &FileGraphics) {
    let FileGraphics {
        cursor: _,
        lines,
        indent_info,
        read_only: _
    } = file;

    print!("\x1b[2J\x1b[H"); // Clear display
    for line in lines {
        match line {
            FileLine::Raw(text) => print!("{text}{}", LINE_TERMINATOR),
            FileLine::Title {
                title, name, title_good, name_good
            } => {
                let title_color = match title_good {
                    true => "\x1b[30;44m",
                    false => "\x1b[30;41m"
                };
                let name_color = match name_good {
                    // name_color could be dependant of the state of the theorem
                    // bad name => red
                    // missing critical parts => grey
                    // incomplete => yellow
                    // valid => green
                    true => "\x1b[30;46m",
                    false => "\x1b[30;41m"
                };
                print!("{title_color}{title} {name_color}{name}{}", LINE_TERMINATOR);
            },
            FileLine::Section { name, is_valid } => {
                let color = match is_valid {
                    true => "",
                    false => ""
                };
                print!("{color}{name}{}", LINE_TERMINATOR);
            },
            FileLine::Hypothesis { name, hypot } => {
                let line_display = colored_string_display(hypot);
                print!("{name}: {line_display}{}", LINE_TERMINATOR);
            },
            FileLine::Assertion { assertion, is_proven } => {
                let color = match is_proven {
                    ProvenState::NotProven => "",
                    ProvenState::Proven => "",
                    ProvenState::Assumed => "",
                    ProvenState::None => ""
                };
                let line_display = colored_string_display(assertion);
                print!("{color}{line_display}{}", LINE_TERMINATOR);
            },
            FileLine::ProofLine {
                line_no, line_index, used_hypots,
                theo_ref, theo_ref_exists, formula
            } => {
                let line_no_color = match line_no.parse::<usize>() == Ok(*line_index) {
                    true => "",
                    false => ""
                };
                let line_no_indent = " ".repeat(indent_info.line_number_indent - line_no.len());

                let used_hypots_display = used_hypots.join(", ");
                let used_hypots_indent = " ".repeat(
                    indent_info.used_hypotheses_indent - used_hypots_display.len()
                );

                let theo_ref_color = match theo_ref_exists {
                    true => "",
                    false => ""
                };
                let theo_ref_indent = " ".repeat(
                    indent_info.theorem_reference_indent - theo_ref.len()
                );

                let formula_display = colored_string_display(formula);
                print!(
                    "{line_no_color}{line_no}{line_no_indent}; \
                     {used_hypots_display}{used_hypots_indent}; \
                     {theo_ref_color}{theo_ref}{theo_ref_indent}; \
                     {formula_display}{}",
                    LINE_TERMINATOR
                )
            },
            FileLine::UnexpectedLine(text) => print!("\x1b[31m{text}{}", LINE_TERMINATOR)
        }
    };
    stdout().flush().unwrap();
}
