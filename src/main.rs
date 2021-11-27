use std::fs::{File, OpenOptions};
use std::io::{BufReader, ErrorKind, Read, Write};
use std::net::Ipv4Addr;
use std::process;
use clap::{App, Arg};

fn replace_slice<T>(buf: &mut [T], from: &[T], to: &[T])
    where
        T: Clone + PartialEq,
{
    for i in 0..=buf.len() - from.len() {
        if buf[i..].starts_with(from) {
            buf[i..(i + from.len())].clone_from_slice(to);
        }
    }
}

fn main() {
    let matches = App::new("Growtopia items.dat decoder")
        .version("1.0")
        .about("This program replace server string in growtopia.exe with ip address given")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("growtopia.exe file path (default: growtopia.exe)"))
        .arg(Arg::with_name("ip")
            .short("i")
            .long("ip")
            .takes_value(true)
            .validator(|s| {
                s.parse::<Ipv4Addr>().map(|_| ()).map_err(|_| String::from("Invalid IP address"))
            })
            .help("IP address to replace with (default: 127.0.0.1)"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Output file path (default: patched.exe)"))
        .get_matches();

    let file_name = matches.value_of("file").unwrap_or("growtopia.exe");
    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let output_file = matches.value_of("output").unwrap_or("patched.exe");

    let file = OpenOptions::new()
        .read(true)
        .open(file_name);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("{} not found, please check file path", file_name);
                process::exit(1);
            },
            _other_error => {
                println!("Problem opening the file, do you have the correct permissions?");
                println!("Perhaps, the file is currently in use");
                process::exit(1);
            }
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer);
    // Replace growtopia1.com
    let host = "growtopia1.com".bytes().collect::<Vec<u8>>();
    let mut ip = ip.bytes().collect::<Vec<u8>>();
    ip.resize(host.len(), 0);
    replace_slice(&mut buffer[..], &*host, &*ip);
    // Replace growtopia2.com
    let host = "growtopia2.com".bytes().collect::<Vec<u8>>();
    replace_slice(&mut buffer[..], &*host, &*ip);
    let mut file = File::create(output_file).unwrap();
    file.write_all(&*buffer);
}
