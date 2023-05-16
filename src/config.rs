use std::fs::{File};
use std::io::{BufRead, BufReader};

pub struct Config {
    // ls settings
    directory_text_color: String,
    filename_text_color: String,
    // error settings
    error_text_color: String,
}

impl Config {
    /// Create config object with default settings
    pub fn new() -> Self {
        Config {
            directory_text_color: String::from("42;125;211"),
            filename_text_color: String::from("192;192;192"),
            error_text_color: String::from("255;0;0"),
        }
    }

    /// Reads config file and settings settings according to read values
    pub fn read_config_file(&mut self) {
        let config_file: File = File::open("config.txt").unwrap();

        let reader: BufReader<&File> = BufReader::new(&config_file);

        for line_result in reader.lines() {
            let line: String = line_result.unwrap();

            if line.contains("#") {
                println!("Found comment");
            } else {
                self.handle_settings(&line);
            }
        }
    }

    /// Handles the parsing of config file text
    fn handle_settings(&mut self, line: &String) {
        let line_values = line.split(":").collect::<Vec<&str>>();

        match line_values[0] {
            "directory_text_color" => self.directory_text_color = line_values[1].to_string(),
            "filename_text_color" => self.filename_text_color = line_values[1].to_string(),
            "error_text_color" => self.error_text_color = line_values[1].to_string(),
            &_ => println!("invalid line found -- {}", line_values[0]),
        }
    }

    pub fn get(&self, field_string: &str) -> String {
        return match field_string {
            "directory_text_color" => self.directory_text_color.clone(),
            "filename_text_color" => self.filename_text_color.clone(),
            "error_text_color" => self.error_text_color.clone(),
            _ => String::from("No value for given field"),
        };
    }
}
