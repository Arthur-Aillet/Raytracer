//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// config
//

#[derive(Debug, Clone)]
pub struct Config {
    pub width: i64,
    pub height: i64,
    pub save_file: String,
    pub config_file: String,
    pub graphic: bool,
    pub layout: bool,
    pub fast_mode: i64,
    pub help: bool,
}

fn config_is_correct(config: &mut Config) -> bool {
    if config.width <= 0 || config.height <= 0 {
        config.help = true;
    }
    if config.save_file.is_empty() || config.config_file.is_empty() {
        config.help = true;
    }
    if config.fast_mode < 0 {
        config.help = true;
    }
    true
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 960,
            height: 540,
            save_file: String::from("scene_example.ppm"),
            config_file: String::from("examples/example.json"),
            graphic: false,
            layout: false,
            fast_mode: 0,
            help: false,
        }
    }
}

impl Config {
    fn get_flag_content(args: &[String], flag: &str) -> Option<String> {
        for (i, arg) in args.iter().enumerate() {
            if arg == flag {
                return if i + 1 < args.len() {
                    Some(args[i + 1].clone())
                } else {
                    None
                };
            }
        }
        None
    }

    fn is_flag(args: &[String], flag: &str) -> bool {
        for arg in args.iter() {
            if arg == flag {
                return true;
            }
        }
        false
    }

    pub fn from_args(args: &[String]) -> Config {
        let mut config = Config::default();

        if let Some(width) = Config::get_flag_content(args, "-w") {
            config.width = width.parse().unwrap_or(config.width);
        }
        if let Some(height) = Config::get_flag_content(args, "-h") {
            config.height = height.parse().unwrap_or(config.height);
        }
        if let Some(save_file) = Config::get_flag_content(args, "-s") {
            config.save_file = save_file;
        }
        if let Some(config_file) = Config::get_flag_content(args, "-j") {
            config.config_file = config_file;
        }
        if let Some(fast) = Config::get_flag_content(args, "-f") {
            config.fast_mode = fast.parse().unwrap_or(config.fast_mode);
        }
        if Config::is_flag(args, "--graphic") {
            config.graphic = true;
        }
        if Config::is_flag(args, "--layout") {
            config.layout = true;
        }
        if Config::is_flag(args, "--help") {
            config.help = true;
        }
        config_is_correct(&mut config);
        config
    }

    pub fn print(&self) {
        println!("+--------------------------------]");
        println!("| Config:");
        println!("|\twidth:\t\t{}", self.width);
        println!("|\theight:\t\t{}\n|", self.height);
        println!("|\tsave_file:\t{}", self.save_file);
        println!("|\tconfig_file:\t{}\n|", self.config_file);
        println!("|\tgraphic:\t{}", self.graphic);
        println!("|\tlayout:\t\t{}", self.layout);
        println!("|\tfast_mode:\t{}", self.fast_mode);
        println!("+----------------------------------------------]");
    }
}
