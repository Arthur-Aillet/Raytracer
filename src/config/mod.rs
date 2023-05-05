//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// config
//

pub struct Config {
    pub width: i64,
    pub height: i64,
    pub save_file: String,
    pub config_file: String,
    pub graphic: bool,
    pub fast_mode: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 960,
            height: 540,
            save_file: String::from("scene_example.ppm"),
            config_file: String::from("example.json"),
            graphic: false,
            fast_mode: false
        }
    }

    fn get_flag(args: &[String], flag: &str) -> Option<String> {
        for (i, arg) in args.iter().enumerate() {
            if arg == flag {
                if i + 1 < args.len() {
                    return Some(args[i + 1].clone());
                } else {
                    return None;
                }
            }
        }
        None
    }

    pub fn from_args(args: &[String]) -> Config {
        let mut config = Config::new();

        if let Some(width) = Config::get_flag(args, "-w") {
            config.width = width.parse().unwrap_or(config.width);
        }
        if let Some(height) = Config::get_flag(args, "-h") {
            config.height = height.parse().unwrap_or(config.height);
        }
        if let Some(save_file) = Config::get_flag(args, "-s") {
            config.save_file = save_file;
        }
        if let Some(config_file) = Config::get_flag(args, "-j") {
            config.config_file = config_file;
        }
        if Config::get_flag(args, "-g").is_some() {
            config.graphic = true;
        }
        if Config::get_flag(args, "-f").is_some() {
            config.fast_mode = true;
        }
        return config;
    }

    pub fn print(&self) {
        println!("---------------------------------");
        println!("Config:");
        println!("\twidth:\t\t{}", self.width);
        println!("\theight:\t\t{}\n", self.height);
        println!("\tsave_file:\t{}", self.save_file);
        println!("\tconfig_file:\t{}\n", self.config_file);
        println!("\tgraphic:\t{}", self.graphic);
        println!("\tfast_mode:\t{}", self.fast_mode);
        println!("---------------------------------");
    }
}
