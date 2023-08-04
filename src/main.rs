use std::io::{Error, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread::{Builder, JoinHandle};
fn main() {
    let addr:&'static str = "localhost:2500";
    let server:TcpListener = TcpListener::bind(addr).unwrap();
    println!("{} thread(s) available(s)",num_cpus::get());
    println!("server will run on {}",server.local_addr().unwrap());
    println!("{:?}",server.take_error().unwrap());
    let mut threadlist: Vec<JoinHandle<()>>=Vec::with_capacity(num_cpus::get());
    loop {
        match server.accept() {
            Ok((stream,address)) => {
                match Builder::new().name(address.port().to_string()).spawn(move || { streamroutine(stream);}) {
                    Ok(join)=> threadlist.push(join),
                    Err(e)=> {println!("{e}")}
            }},
            Err(_) => {}
        }
        threadlist.sort_by_key(|thread| thread.is_finished());
        while threadlist.last().is_some_and(|t| t.is_finished()) {
            threadlist.pop().is_some_and(|t| {t.join();true});
        }
        println!("eol")
    }
}
/*
fn handle_connection(flux:Result<TcpStream,Error>) {
    match flux {
        Ok(val) => streamroutine(val),
        Err(e) => panic!("{e}")
    }
}
*/

fn streamroutine(mut stream:TcpStream) {
    println!("{}",stream.peer_addr().unwrap());
    let mut input: Vec<u8> = Vec::new();
    if stream.read(&mut input).is_ok() {
        println!("{:?}", input)
    }
    stream.write(&[1]);
    stream.shutdown(std::net::Shutdown::Both);
}