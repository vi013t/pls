use colored::Colorize as _;
use std::io::Write as _;

const BASH_INIT: &str = include_str!("../scripts/init.bash");

fn main() {
    let command_line_arguments = <CommandLineArguments as clap::Parser>::parse();

    if command_line_arguments.init {
        println!("{BASH_INIT}");
        return;
    }

    // Print current directory
    print!("\x1B[2J\x1B[1;1H");
    println!("┌ {}", format!("{}:", std::env::current_dir().unwrap().display()).bold().white());
    println!("{}", "│");

    // Get files
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for file in std::fs::read_dir(std::env::current_dir().unwrap()).unwrap().filter_map(Result::ok) {
        let path = file.path();
        let file_name = file.file_name().into_string().unwrap();
        if path.is_file() {
            files.push((file_name, path));
        } else {
            dirs.push(file_name);
        }
    }

    // Print directories
    let (r, g, b) = hex_to_rgb(&command_line_arguments.directory_color).unwrap();
    for dir in dirs {
        if command_line_arguments.show_dotfiles || !dir.starts_with(".") {
            println!("│ {} {}", "".truecolor(r, g, b), dir);
        }
    }

    // Print files
    for (file, path) in files {
        let icon = devicons::icon_for_file(&devicons::File::new(&path), command_line_arguments.theme.some_devicon_theme());
        let (r, g, b) = hex_to_rgb(icon.color).unwrap();
        if command_line_arguments.show_dotfiles || !file.starts_with(".") {
            println!("│ {} {}", format!("{}", icon.icon).truecolor(r, g, b), file);
        }
    }

    // Print blank line
    if std::env::consts::OS == "windows" {
        print!("│");
        std::io::stdout().flush().unwrap();
    } else {
        println!("│");
    };
}

#[derive(clap::Parser)]
struct CommandLineArguments {
    #[arg(short = 'd', long)]
    show_dotfiles: bool,

    #[arg(short = 'c', long, default_value = "#89b4fa")]
    directory_color: String,

    #[arg(short, long, default_value = "dark")]
    theme: Theme,

    #[arg(long)]
    init: bool,
}

#[derive(Clone, clap::ValueEnum)]
enum Theme {
    Dark,
    Light,
}

impl Theme {
    fn some_devicon_theme(&self) -> Option<devicons::Theme> {
        Some(match self {
            Self::Light => devicons::Theme::Light,
            Self::Dark => devicons::Theme::Dark,
        })
    }
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    // Remove the '#' if present
    let hex = hex.trim_start_matches('#');

    // Enforce correct string length
    if hex.len() != 6 && hex.len() != 3 {
        return Err("Invalid hex color length".to_string());
    }

    // If the hex is 3 characters long, convert it to 6 characters
    let hex = if hex.len() == 3 {
        hex.chars().map(|c| format!("{c}{c}", c = c)).collect::<String>()
    } else {
        hex.to_string()
    };

    // Parse the hex string to u8 values
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex value".to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex value".to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex value".to_string())?;

    Ok((r, g, b))
}
