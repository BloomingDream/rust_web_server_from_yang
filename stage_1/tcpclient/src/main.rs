use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};
fn main() {
    let mut _stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    _stream.write("hello".as_bytes()).unwrap();
    let mut buf = [0; 5];
    _stream.read(&mut buf).unwrap();
    println!("{}", str::from_utf8(&buf).unwrap());
}
