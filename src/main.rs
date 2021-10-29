use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::read;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::thread;
use std::time::{Duration, Instant};
use termion::{color, style};

fn main() {
    let matches = App::new("simple_socket_tester")
        .version("0.1.0")
        .author("Matvei Klimov <klimatt.gu@gmail.com>")
        .about("Simple socket tester")
        .arg(Arg::new("ip").short("i".parse().unwrap()).takes_value(true))
        .arg(
            Arg::new("port")
                .short("p".parse().unwrap())
                .takes_value(true),
        )
        .arg(
            Arg::new("max_packet_size") // bytes
                .short("m".parse().unwrap())
                .takes_value(true),
        )
        .arg(
            Arg::new("delay") // ms
                .short("d".parse().unwrap())
                .takes_value(true),
        )
        .get_matches();
    println!("Simple Socket Tester");

    let ip_addr = matches.value_of("ip").unwrap();
    let port = matches.value_of("port").unwrap();
    let max_packet_size = matches
        .value_of("max_packet_size")
        .unwrap()
        .parse::<usize>()
        .expect("");
    let delay = matches
        .value_of("max_packet_size")
        .unwrap()
        .parse::<u64>()
        .expect("");
    let t = Instant::now();
    loop {
        match TcpStream::connect(ip_addr.to_owned() + ":" + port) {
            Ok(mut stream) => {
                println!(
                    "{}{:?} | {}Successfully connected to {} in port {} {}",
                    color::Fg(color::Cyan),
                    t.elapsed(),
                    color::Fg(color::Green),
                    ip_addr,
                    port,
                    color::Fg(color::White)
                );
                let mut rng = rand::thread_rng();
                let bytes_amount = rng.gen_range(0usize..max_packet_size);
                let rand_string: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(bytes_amount)
                    .map(char::from)
                    .collect();

                stream.write(rand_string.as_bytes()).unwrap();
                println!(
                    "{}{:?} |{} Sent {}{}, {}awaiting reply...",
                    color::Fg(color::Cyan),
                    t.elapsed(),
                    color::Fg(color::Green),
                    color::Fg(color::Yellow),
                    rand_string,
                    color::Fg(color::White)
                );

                let mut data = Vec::<u8>::new();
                data.resize(bytes_amount, 0);
                match stream.read_exact(&mut data[0..bytes_amount]) {
                    Ok(_) => {
                        if data.as_slice() == rand_string.as_bytes() {
                            println!(
                                "{}{:?} | {}Reply is ok!",
                                color::Fg(color::Cyan),
                                t.elapsed(),
                                color::Fg(color::Green)
                            );
                        } else {
                            println!(
                                "{}{:?} | {} Unexpected reply: {}{:?}",
                                color::Fg(color::Cyan),
                                t.elapsed(),
                                color::Fg(color::Red),
                                color::Fg(color::Yellow),
                                data
                            );
                        }
                    }
                    Err(e) => {
                        println!(
                            "{}{:?} | {}Failed to receive data: {}{}",
                            color::Fg(color::Cyan),
                            t.elapsed(),
                            color::Fg(color::Red),
                            color::Fg(color::Yellow),
                            e
                        );
                    }
                }
                stream.shutdown(Shutdown::Both);
            }
            Err(e) => {
                println!(
                    "{}{:?} | {}Failed to connect: {}{}",
                    color::Fg(color::Cyan),
                    t.elapsed(),
                    color::Fg(color::Red),
                    color::Fg(color::Yellow),
                    e
                );
            }
        }

        thread::sleep(Duration::from_millis(delay));
    }
}
