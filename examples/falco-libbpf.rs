extern crate libbpf_rs;

use std::env;
use std::process;

use libbpf_rs::ObjectBuilder;

fn usage() -> ! {
    println!("{} <bpf-probe.o>", env::args().next().unwrap());
    process::exit(1);
}

fn main() {
    let path = env::args().skip(1).next();

    let mut object_builder = ObjectBuilder::default();

    match path {
        Some(path) => {
            let open_object = object_builder.open_file(path).unwrap();
            let _object = open_object.load().unwrap();
        }
        None => usage(),
    }
}
