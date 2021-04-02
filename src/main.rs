#[macro_use]
extern crate clap;
extern crate tokio;
use clap::App;
use std::error::Error;


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
    Ok(())
}
