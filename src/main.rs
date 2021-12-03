extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod enigma;
mod error;
mod physical_rotor;
mod reflector;
mod rotor;

fn main() {
    pretty_env_logger::init();

    println!("Hello, world!");
}
