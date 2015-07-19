#[macro_use] extern crate nickel;

use nickel::Nickel;

mod routes;

fn main() {
    let mut server = Nickel::new();

    routes::build_routes(&mut server);

    server.listen("127.0.0.1:6767");
}

