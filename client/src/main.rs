mod client_config;

#[macro_use]
extern crate clap;

use clap::App;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;
use crate::client_config::ClientConfig;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build()?;

    let mut config = get_config();

    if !dir_exists("./tmp") {
        std::fs::create_dir("./tmp");
    }


    let yaml = load_yaml!("cli.yaml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    if let Some(install) = matches.subcommand_matches("install") {
        let pkg = match install.value_of("INPUT") {
            None => {
                println!("Expected value for INPUT, run 'gib install --help' for help");
                return Ok(()); }
            Some(e) => e
        }.split("@").map(|e| e.to_owned()).collect::<Vec<String>>();
        if pkg.len() == 1 {
            println!("Finding package {}@latest", &pkg[0]);

            if config.repos.is_empty() {
                eprintln!("Error, please add a repo, run 'gib add_repo --help' for help");
                return Ok(());
            }

            let mut package_found = false;

            for repo in config.repos.iter() {
                let url = format!("{}pkg/get/{}@latest", repo, pkg[0]);
                println!("=> Checking {}", url);
                let response = client.get(url.as_str()).send().await?;
                if response.status() == 200 {
                    println!("=> Downloading from: {}", url);
                    package_found = true;

                    let binary = response.bytes().await?;

                    let mut output = std::fs::File::create("./tmp/output.tar.gz")?;
                    output.write_all(binary.as_ref())?;
                    println!("=> Downloaded tarball");
                    drop(output);

                    println!("=> Extracting tarball");
                    let mut output = std::fs::File::open("./tmp/output.tar.gz")?;
                    let mut archive = Archive::new(GzDecoder::new(output.try_clone()?));

                    let paths = archive
                        .entries()?
                        .filter_map(|e| e.ok())
                        .map(|mut entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
                            let path = entry.path()?.strip_prefix("")?.to_owned();
                            entry.unpack(&path)?;
                            Ok(path)
                        })
                        .filter_map(|e| e.ok())
                        .collect::<Vec<PathBuf>>();

                    drop(archive);

                    let mut archive = Archive::new(GzDecoder::new(output));
                    archive.unpack("./tmp");

                    let mut working_dir = "./tmp".to_string();

                    if paths.get(0).unwrap().is_dir() {
                        working_dir = format!("./tmp/{}", paths.get(0).unwrap().display());
                    }

                    println!("{}", working_dir);

                    break;
                }
            }

            if !package_found {
                println!("No package found for {}@latest", &pkg[0]);
            }
        }
    } else if let Some(add_repo) = matches.subcommand_matches("add_repo") {
        let repo = format!("https://{}/", add_repo.value_of("INPUT").unwrap());

        println!("Adding repo {}", repo);

        match client.get(repo.as_str()).send().await {
            Ok(_) => {
                config.repos.push(repo.to_string());
                println!("Added repo {}", repo);
            }
            Err(e) => {
                eprintln!("Error accessing repo {}\nError: {}", repo, e);
            }
        }

        write_config(&config);
    }

    std::fs::remove_dir_all("./tmp")?;

    Ok(())
}

fn write_config(config: &ClientConfig) {
    let home = std::env::var("HOME").unwrap();

    if !dir_exists(format!("{}/.config", home).as_str()) {
        std::fs::create_dir(format!("{}/.config", home).as_str()).unwrap();
    }

    if !dir_exists(format!("{}/.config/gib", home).as_str()) {
        std::fs::create_dir(format!("{}/.config/gib", home).as_str()).unwrap();
    }

    let mut file = std::fs::File::create(format!("{}/.config/gib/config.toml", home).as_str()).unwrap();

    let config_string = toml::to_string(config).unwrap();
    file.write_all(config_string.as_bytes()).unwrap();
}

fn get_config() -> ClientConfig {
    let home = std::env::var("HOME").unwrap();

    if !dir_exists(format!("{}/.config", home).as_str()) {
        std::fs::create_dir(format!("{}/.config", home).as_str()).unwrap();
    }

    if !dir_exists(format!("{}/.config/gib", home).as_str()) {
        std::fs::create_dir(format!("{}/.config/gib", home).as_str()).unwrap();
    }

    if !file_exists(format!("{}/.config/gib/config.toml", home).as_str()) {
        let mut file = std::fs::File::create(format!("{}/.config/gib/config.toml", home).as_str()).unwrap();
        let new_config = ClientConfig {
            installed: vec!(),
            repos: vec!(),
        };
        let config_string = toml::to_string(&new_config).unwrap();
        file.write_all(config_string.as_bytes()).unwrap();
        return new_config;
    }

    let mut config_file = std::fs::File::open(format!("{}/.config/gib/config.toml", home).as_str()).unwrap();
    let mut config_file_str = String::new();
    config_file.read_to_string(&mut config_file_str);
    let conf = toml::from_str::<ClientConfig>(config_file_str.as_str()).unwrap();

    return conf;
}

fn dir_exists(dir: &str) -> bool {
    Path::new(dir).is_dir()
}

fn file_exists(file: &str) -> bool {
    Path::new(file).is_file()
}
