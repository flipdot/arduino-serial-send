extern crate clap;
//extern crate daemonize;
extern crate serial;

use std::fs::File;
use std::io::{BufReader, BufRead, Write, stderr};
use std::process::exit;

use clap::{App, Arg};
//use daemonize::Daemonize;
use serial::SerialPort;

fn main() {
    let matches = App::new("arduino-serial-send")
        .version("1.0")
        .about("Daemon that keeps a serial port alive and forwards messages to it from a named \
                pipe / fifo.")
        .arg(Arg::with_name("fifo")
            .long("fifo")
            .help("Custom fifo file path (default: /tmp/send_to_arduino)")
            .takes_value(true))
        .arg(Arg::with_name("serial")
            .long("serial")
            .help("Serial port path")
            .takes_value(true))
        .get_matches();

    let fifo_path = matches.value_of("socket").unwrap_or("/tmp/send_to_arduino");
    let serial_path = match matches.value_of("serial") {
        Some(p) => p,
        None => {
            writeln!(&mut stderr(), "Missing option serial!").unwrap();
            //writeln!(&mut stderr(), "{}", matches.usage()).unwrap();
            exit(1)
        }
    };

    let mut serial = match serial::open(serial_path) {
        Ok(s) => s,
        Err(_) => {
            writeln!(&mut stderr(),
                     "Couldn't open serial port '{}'.",
                     serial_path)
                .unwrap();
            exit(1)
        }
    };

    if serial.reconfigure(&|settings| settings.set_baud_rate(serial::Baud9600)).is_err() {
        writeln!(&mut stderr(),
                 "Couldn't configure serial port '{}'.",
                 serial_path)
            .unwrap();
        exit(1);
    }

    // TODO: non-stderr logging
    // Daemonize::new().start();

    loop {
        // Reopen the fifo after every EOF - not a very nice solution! [TODO]
        let fifo = match File::open(fifo_path) {
            Ok(s) => s,
            Err(_) => {
                writeln!(&mut stderr(), "Couldn't open fifo '{}'.", fifo_path).unwrap();
                exit(1)
            }
        };

        let fifo_reader = BufReader::new(fifo);

        for line in fifo_reader.lines() {
            serial.write_all(line.unwrap().as_bytes()).unwrap();
        }
    }
}
