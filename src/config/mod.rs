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
    pub graphic: i64,
    pub fast_mode: i64,
    pub help: bool,
}

fn config_is_correct(mut config: &mut Config) -> bool {
    if config.width <= 0 || config.height <= 0 {
        config.help = true;
    }
    if config.save_file == "" || config.config_file == "" {
        config.help = true;
    }
    if config.graphic < 0 || config.graphic > 2 {
        config.help = true;
    }
    if config.fast_mode < 0 {
        config.help = true;
    }
    true
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 960,
            height: 540,
            save_file: String::from("scene_example.ppm"),
            config_file: String::from("example.json"),
            graphic: 0,
            fast_mode: 0,
            help: false,
        }
    }

    fn get_flag_content(args: &[String], flag: &str) -> Option<String> {
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

    fn is_flag(args: &[String], flag: &str) -> bool {
        for arg in args.iter() {
            if arg == flag {
                return true;
            }
        }
        false
    }

    pub fn from_args(args: &[String]) -> Config {
        let mut config = Config::new();

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
        if let Some(graphic) = Config::get_flag_content(args, "-g") {
            config.graphic = graphic.parse().unwrap_or(config.graphic);
        }
        if Config::is_flag(args, "--help") {
            config.help = true;
        }
        config_is_correct(&mut config);
        return config;
    }

    pub fn print(&self) {
        println!("+--------------------------------]");
        println!("| Config:");
        println!("|\twidth:\t\t{}", self.width);
        println!("|\theight:\t\t{}\n|", self.height);
        println!("|\tsave_file:\t{}", self.save_file);
        println!("|\tconfig_file:\t{}\n|", self.config_file);
        println!("|\tgraphic:\t{}", self.graphic);
        println!("|\tfast_mode:\t{}", self.fast_mode);
        println!("+----------------------------------------------]");
    }
}
