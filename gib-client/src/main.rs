#[macro_use]
extern crate clap;

use clap::App;
use std::io::{Write, Read};
use std::path::{Path, PathBuf, Display};
use flate2::read::GzDecoder;
use tar::{Archive, Entries, Entry};
use gib_common::config::{ClientConfig, Package, PackageConfig};
use std::process::exit;
use std::fs::{File, metadata};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build()?;

    let mut config = get_config();

    if !dir_exists("./tmp") {
        fs::create_dir("./tmp");
    }

    let yaml = load_yaml!("cli.yaml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    if let Some(install) = matches.subcommand_matches("install") {
        let pkg = match install.value_of("INPUT") {
            None => {
                println!("Expected value for INPUT, run 'gib install --help' for help");
                return Ok(());
            }
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

                    let mut output = fs::File::create("./tmp/output.tar.gz")?;
                    output.write_all(binary.as_ref())?;
                    println!("=> Downloaded tarball");
                    drop(output);

                    println!("=> Extracting tarball");
                    let mut output = fs::File::open("./tmp/output.tar.gz")?;
                    let mut archive = Archive::new(GzDecoder::new(output));
                    archive.unpack("./tmp").unwrap();

                    let mut archive =  Archive::new(GzDecoder::new(File::open("./tmp/output.tar.gz").unwrap()));
                    let indexes = archive
                        .entries()
                        .unwrap()
                        .collect::<Vec<Result<Entry<GzDecoder<File>>, std::io::Error>>>();

                    let mut working_dir = "./tmp".to_string();

                    match indexes.get(0) {
                        Some(e) => { working_dir = format!("./tmp/{}", e.as_ref().unwrap().path().unwrap().display()) }
                        None => { eprintln!("Nothing in the archive ¯\\_(ツ)_/¯"); }
                    }

                    let list = std::path::Path::new(format!("{}extract", working_dir.clone()).as_str()).read_dir().unwrap();

                    let home = std::env::var("HOME")?;

                    for path in list {
                        let path = path.unwrap().path();

                        copy(path.clone(), format!("{}/{}", home.clone(), path.clone().strip_prefix(format!("{}extract", working_dir.clone()).as_str()).unwrap().display()))?;
                    }

                    let package_config = read_pkg_config_file(format!("{}/package.toml", working_dir).as_str())?;

                    fs::copy(format!("{}/package.toml", working_dir).as_str(), format!("{}/.config/gib/installed/{}@{}.toml", home, &package_config.name, &package_config.version));

                    println!("   copy: {}package.toml -> {}/.config/gib/installed/{}@{}.toml", working_dir, home, &package_config.name, &package_config.version);

                    config.installed.push(Package {
                        repo: repo.to_string(),
                        name: package_config.clone().name,
                        version: package_config.clone().version
                    });

                    println!("=> Installed {}@{}", package_config.name, package_config.version);

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

    fs::remove_dir_all("./tmp")?;

    Ok(())
}

fn write_config(config: &ClientConfig) {
    let home = std::env::var("HOME").unwrap();

    if !dir_exists(format!("{}/.config", home).as_str()) {
        fs::create_dir(format!("{}/.config", home).as_str()).unwrap();
    }

    if !dir_exists(format!("{}/.config/gib", home).as_str()) {
        fs::create_dir(format!("{}/.config/gib", home).as_str()).unwrap();
    }

    let mut file = fs::File::create(format!("{}/.config/gib/config.toml", home).as_str())
        .unwrap();

    let config_string = toml::to_string(config).unwrap();
    file.write_all(config_string.as_bytes()).unwrap();
}

fn get_config() -> ClientConfig {
    let home = std::env::var("HOME").unwrap();

    if !dir_exists(format!("{}/.config", home).as_str()) {
        fs::create_dir(format!("{}/.config", home).as_str()).unwrap();
    }

    if !dir_exists(format!("{}/.config/gib", home).as_str()) {
        fs::create_dir(format!("{}/.config/gib", home).as_str()).unwrap();
    }

    if !dir_exists(format!("{}/.config/gib/installed", home).as_str()) {
        fs::create_dir(format!("{}/.config/gib/installed", home).as_str()).unwrap();
    }

    if !file_exists(format!("{}/.config/gib/config.toml", home).as_str()) {
        let mut file = fs::File::create(format!("{}/.config/gib/config.toml", home).as_str()).unwrap();
        let new_config = ClientConfig {
            installed: vec!(),
            repos: vec!(),
        };
        let config_string = toml::to_string(&new_config).unwrap();
        file.write_all(config_string.as_bytes()).unwrap();
        return new_config;
    }

    let mut config_file = fs::File::open(format!("{}/.config/gib/config.toml", home).as_str()).unwrap();
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

// Copied from stackoverflow: https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("   copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn read_pkg_config_file(file: &str) -> Result<PackageConfig, Box<dyn std::error::Error>>{
    let mut buff = String::new();
    let mut file = File::open(file)?;
    file.read_to_string(&mut buff)?;
    Ok(toml::from_str::<PackageConfig>(&buff)?)
}
