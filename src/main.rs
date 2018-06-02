extern crate tig_100;

use std::process::exit;

fn main() {
    if let Err(e) = tig_100::run() {
        eprintln!("{}", e);
        exit(1);
    }
}
