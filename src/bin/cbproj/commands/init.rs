use argh::FromArgs;

use std::collections::HashMap;
use std::fs;
use std::env;

use serde::{Deserialize, Serialize};
use dialoguer::Input;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub main: String,
    pub repository: Option<String>,
    pub license: Option<String>,
    #[serde(default)]
    pub private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bin {
    pub name: String,
    pub entry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CBProj {
    pub project: Project,
    #[serde(default)]
    pub bin: Option<Vec<Bin>>,
    #[serde(default)]
    pub dependencies: Option<HashMap<String, Dependency>>,
    #[serde(default)]
    pub authors: Option<Vec<Author>>,
}

#[derive(FromArgs, Debug)]
/// Initializes a cbproj.toml file.
#[argh(subcommand, name = "init")]
pub struct Init {}

pub fn run() {
    // Ask the following questions:
    // Project name
    // Project version (default 0.1.0)
    // Project main file (default main.cb)
    // Project repo (default none)
    // Project license (default MIT)
    // Will not ask if the project is private (will just default to true)

    // current working directory to place cbproj.toml in
    let current_dir = env::current_dir().expect("Failed to find the current working directory, required to write the cbproj.toml file.");
    let current_dir_name = current_dir.file_name()
        .and_then(|name| name.to_str())
        .expect("Failed to find the name of the current working directory, needed for initialization prompts.");

    let name: String = Input::new()
        .with_prompt("Project name")
        .default(current_dir_name.to_string())
        .interact_text()
        .unwrap();

    let version: String = Input::new()
        .with_prompt("Project version")
        .default("0.1.0".to_string())
        .interact_text()
        .unwrap();

    let main_file: String = Input::new()
        .with_prompt("Project's entry file")
        .default("main.cb".to_string())
        .interact_text()
        .unwrap();

    let repo: String = Input::new()
        .with_prompt("Project repository link")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let license: String = Input::new()
        .with_prompt("Project license")
        .allow_empty(true)
        .default("MIT".to_string())
        .interact_text()
        .unwrap();

    // repo, but if the repo var is empty the value of this var will be None, it's better for serializing in the cbproj.toml file
    let repo_asopt: Option<String> = if repo.is_empty() { None } else { Some(repo) };
    let license_asopt: Option<String> = if license.is_empty() { None } else { Some(license) };

    let project = Project {
        name,
        version,
        main: main_file,
        repository: repo_asopt,
        license: license_asopt,
        private: true
    };



    let cbproj: CBProj = CBProj {
        project,
        bin: None,
        authors: None,
        dependencies: None
    };

    let toml_cbproj = toml::to_string_pretty(&cbproj).unwrap();
    let cbproj_path = current_dir.join("cbproj.toml");

    fs::write(cbproj_path, toml_cbproj).expect("Failed to write the cbproj.toml file.");
    println!("cbproj.toml file created!");
}