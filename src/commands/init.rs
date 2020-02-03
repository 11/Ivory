use std::fs;
use super::command::Command;

static INIT_DIRECTORIES: &[&str] = &[
    "templates",
    "posts",
    "static",
    "static/css",
    "static/js",
    "static/images",
];

static INIT_FILES: &[&str] = &[
    ".oxygenrc",
];

pub struct InitCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl Command for InitCommand {
    fn new(command: String, args: Vec<String>) -> InitCommand {
        return InitCommand{command, args};
    }

    fn validate(&self) -> bool {
        return true;
    }

    fn run(&self) {
        let mut directory_builder = fs::DirBuilder::new();
        directory_builder.recursive(true);

        for directory in INIT_DIRECTORIES.iter() {
            let result = directory_builder.create(*directory);
            if let Err(message) = result {
                println!("Error creating directory {} due to {}", directory, message);
                break;
            }
        }

        for file in INIT_FILES.iter() {
            let result = fs::File::create(file);
            if let Err(message) = result {
                println!("Error creating file {} due to {}", file, message);
            }
            break;
        }
    }
}
