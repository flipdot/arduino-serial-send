extern crate clap;
// TODO: daemonize
//extern crate daemonize;
// TODO: replace socket with named fifo
extern crate unix_socket;
extern crate serial;

use clap::{App, Arg};
use unix_socket::UnixListener;
use std::process::exit;
use std::io::{Read, Write, stderr};

fn main() {
    let matches = App::new("arduino-serial-send")
        .version("1.0")
        .about("Daemon that keeps a serial port alive and forwards messages to it from a socket \
                file.")
        .arg(Arg::with_name("socket")
            .long("sock")
            .help("Custom socket file path (default: /tmp/send_to_arduino)")
            .takes_value(true))
        .arg(Arg::with_name("serial")
            .long("serial")
            .help("Serial port path")
            .takes_value(true))
        .get_matches();

    let socket_path = matches.value_of("socket").unwrap_or("/tmp/send_to_arduino");
    let serial_path = match matches.value_of("serial") {
        Some(p) => p,
        None => {
            writeln!(&mut stderr(), "Missing option serial!").unwrap();
            //writeln!(&mut stderr(), "{}", matches.usage()).unwrap();
            exit(1)
        }
    };

    let socket = match UnixListener::bind(socket_path) {
        Ok(s) => s,
        Err(_) => {
            writeln!(&mut stderr(), "Couldn't create socket '{}'.", socket_path).unwrap();
            exit(1)
        }
    };

    // This part error on rail.fd right now :(
    let mut serial = match serial::open(socket_path) {
        Ok(s) => s,
        Err(_) => {
            writeln!(&mut stderr(),
                     "Couldn't open serial port '{}'.",
                     serial_path)
                .unwrap();
            exit(1)
        }
    };

    loop {
        let (mut stream, _) = socket.accept().unwrap();
        let mut buf = String::new();

        if let Err(_) = stream.read_to_string(&mut buf) {
            continue;
        };

        serial.write(buf.as_bytes()).unwrap();
    }
}
