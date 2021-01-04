#[macro_use]
extern crate clap;
extern crate serde_yaml;

use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use clap::Arg;
use serde::Deserialize;

fn default_user() -> String {
    "root".to_owned()
}

fn default_port() -> i64 {
    22
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServerMapping {
    servers: Vec<Server>,
    groups: Vec<ServerGroup>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServerGroup {
    name: String,
    servers: Vec<Server>,
    identity_file: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Server {
    name: String,
    host: String,

    #[serde(default = "default_user")]
    user: String,

    #[serde(default = "default_port")]
    port: i64,

    #[serde(default)]
    forwarding: bool,

    #[serde(default)]
    identity_file: String,
}

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("group")
                .short("g")
                .long("group")
                .value_name("GROUP")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("execute")
                .short("e")
                .long("execute")
                .value_name("EXECUTE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .help("List available servers")
                .short("l")
                .long("list"),
        )
        .arg(
            Arg::with_name("name")
                .help("Sets the server name")
                .required_unless("list")
                .conflicts_with("list")
                .index(1),
        )
        .get_matches();

    let execute_command = matches.value_of("execute").unwrap_or_else(|| "");
    let config_group = matches.value_of("group");
    let config_name = matches.value_of("name");
    let config_list = matches.is_present("list");

    let config_path = if let Some(config_path) = matches.value_of("config") {
        PathBuf::from(config_path)
    } else {
        dirs::config_dir()
            .unwrap_or_else(PathBuf::new)
            .join("sshh.yml")
    };

    let f = File::open(config_path).expect("Unable to open file");
    let server_mapping: ServerMapping =
        serde_yaml::from_reader(f).expect("Unable to deserialize config file");

    let mut server_found: Option<Server> = None;

    if config_group.is_none() {
        for server in server_mapping.servers {
            if config_list {
                println!("  - {}", server.name);
                continue;
            }

            if let Some(config_name) = config_name {
                if config_name == server.name {
                    server_found = Some(server);
                }
            }
        }
    }

    for group in server_mapping.groups {
        if let Some(config_group) = config_group {
            if config_group != group.name {
                continue;
            }
        }

        if config_list {
            println!("{}:", group.name);
        }

        for mut server in group.servers {
            if config_list {
                println!("  - {}", server.name);
                continue;
            }

            if let Some(config_name) = config_name {
                if config_name == server.name {
                    if server.identity_file.is_empty() {
                        if let Some(ref group_identity) = group.identity_file {
                            server.identity_file = group_identity.to_string();
                        }
                    }
                    server_found = Some(server);
                }
            }
        }
    }

    if let Some(server) = server_found {
        println!(
            "Connecting to '{}' via {}@{}...",
            server.name, server.user, server.host
        );

        let mut args = vec![];

        if server.forwarding {
            args.push("-A");
        }

        if !server.identity_file.is_empty() {
            args.push("-i");
            args.push(&server.identity_file);
        }

        Command::new("ssh")
            .args(args)
            .arg("-p")
            .arg(format!("{}", server.port))
            .arg(format!("{}@{}", server.user, server.host))
            .arg(execute_command)
            .status()
            .expect("failed to execute process");
    }
}
