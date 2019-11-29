# 🖥️ sshh

_(aka `ssh hosts`)_ - Quickly SSH into hosts

Problem:

* dozens of hosts to SSH in to but I can never remember the particular IP
* cannot use domain name because they sit behind Cloudflare
* unable to use subdomain because it can expose the IP
* `.ssh/config` doesn't support grouping of servers

## Installation

For now:

(You will need the Rust compiler installed)

1. Clone the repository and build using `cargo build --release`
1. Install using `cargo install --path .`

I am working on releasing binaries/packages.

## Usage

```bash
$ sshh project-server # connect to the project-server

$ sshh main # connect to the last `main` host specified

$ sshh -g acme-corp main # connect to the `main` host under the acme-corp group
```

Given an ambiguous host name, `sshh` will connect to the last one defined in the config file.

`sshh` ends up calling `ssh` with the suitable arguments, so make sure it is installed.

## Hosts config

`sshh` uses the yaml file format, generally stored in `~/.config/sshh.yml`. You can pass a custom config through the `-c` flag.

A server has the following options:

* name: the server name
* user (optional): defaults to "root"
* host: the domain or IP address
* port (optional): defaults to 22
* forwarding (optional): whether to enable authentication agent forwarding, defaults to false

Servers can be put into groups, or listed separately. List individual servers under the `servers` mapping, and groups under `groups`. The config file is deserialized using [`serde_yaml`](https://docs.rs/serde_yaml/). The example below should give a good indication of the expected format.

```yml
servers:
    - name: project-server
      user: banana
      host: 1.2.3.4
      port: 1337
      forwarding: true
    - name: vpn
      host: 2.3.4.5
groups:
    - name: random-company
      servers:
          - name: main
            user: app
            host: random-company.com
          - name: backup
            user: app
            host: backup.random-company.com
    - name: acme-corp
      servers:
          - name: main
            host: main.acme-corp.com
          - name: venus
            host: venus.acme-corp.com
```
