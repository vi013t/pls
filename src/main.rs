use colored::Colorize as _;
use serde_inline_default::serde_inline_default;
use std::{io::Write as _, path::Path};

#[cfg(windows)]
use std::os::windows::fs::MetadataExt as _;

const BASH_INIT: &str = include_str!("../scripts/init.bash");

fn main() -> anyhow::Result<()> {
    let command_line_arguments = <CommandLineArguments as clap::Parser>::parse();
    let config: Config = get_config()?;

    if command_line_arguments.init {
        println!("{BASH_INIT}");
        return Ok(());
    }

    let directory = command_line_arguments
        .directory
        .as_ref()
        .and_then(|dir| Some(shellexpand::full(dir).unwrap().into_owned()))
        .unwrap_or_else(|| std::env::current_dir().unwrap().display().to_string());

    // Print current directory
    println!("┌ {}", format!("{directory}:").bold());
    println!("{}", "│");

    // Get files
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for file in std::fs::read_dir(&directory).unwrap().filter_map(Result::ok) {
        let path = file.path();
        let file_name = file.file_name().into_string().unwrap();
        if path.is_file() {
            files.push((file_name, path));
        } else {
            dirs.push((file_name, path));
        }
    }

    // Print directories
    let (r, g, b) = hex_to_rgb(&config.directory_color).unwrap();
    for (dir, dir_path) in dirs {
        if config.show(&dir_path) {
            println!("│ {} {}", "".truecolor(r, g, b), dir);
        }
    }

    // Print files
    for (file, path) in files {
        let icon = devicons::icon_for_file(&devicons::File::new(&path), command_line_arguments.theme.some_devicon_theme());
        let (r, g, b) = hex_to_rgb(icon.color).unwrap();
        if config.show(&path) {
            println!("│ {} {}", format!("{}", icon.icon).truecolor(r, g, b), file);
        }
    }

    // Print blank line
    if std::env::var("SHELL").is_err() {
        print!("│");
        std::io::stdout().flush().unwrap();
    } else {
        println!("│");
    };

    Ok(())
}

fn get_config() -> anyhow::Result<Config> {
    let path = format!("{}/.config/pls/pls.toml", std::env::var("HOME")?);
    let config_file_path = Path::new(&path);
    if !config_file_path.is_file() {
        return Ok(Config::default());
    }

    let text = std::fs::read_to_string(config_file_path)?;
    let config: Config = toml::from_str(&text)?;

    Ok(config)
}

fn is_hidden<P: AsRef<Path>>(path: P) -> bool {
    if cfg!(windows) {
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
        if path.as_ref().metadata().unwrap().file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0 {
            return true;
        }
    }

    path.as_ref().file_name().is_some_and(|name| name.to_str().unwrap().starts_with("."))
}

#[derive(clap::Parser)]
struct CommandLineArguments {
    #[arg(short, long, default_value = "dark")]
    theme: Theme,

    #[arg()]
    directory: Option<String>,

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
        return Err(format!("Invalid hex color length: #{hex}"));
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

#[serde_inline_default]
#[derive(serde::Deserialize, Debug)]
struct Config {
    #[serde(default)]
    hide: Vec<String>,

    #[serde(default)]
    show_dotfiles: bool,

    #[serde_inline_default("#89b4fa".to_owned())]
    directory_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            directory_color: "#89b4fa".to_owned(),
            hide: Vec::new(),
            show_dotfiles: false,
        }
    }
}

impl Config {
    fn show<P: AsRef<Path>>(&self, path: P) -> bool {
        if !self.show_dotfiles && is_hidden(&path) {
            return false;
        }

        for hidden_glob in &self.hide {
            let pattern = shellexpand::full(hidden_glob).unwrap();
            let glob = globset::Glob::new(&pattern).unwrap();
            let matcher = glob.compile_matcher();
            if matcher.is_match(&path) {
                return false;
            }
        }

        true
    }
}
