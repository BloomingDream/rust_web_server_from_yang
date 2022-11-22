mod handler;
mod router;
mod server;
use server::Server;
fn main() {
    println!("Hello, world!");
    let server = Server::new("localhost:3000");
    server.run();
}
