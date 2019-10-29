extern crate pkg_config;

fn main() {
    match pkg_config::find_library("libsystemd") {
        Ok(_) => println!("libsystemd found"),
        Err(_) => println!("libsystemd NOT found"),
    };
}
