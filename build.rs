use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/*");
    copy_to_output("assets", &env::var("PROFILE").unwrap())
        .expect("Unable to copy files from assets folder");
}
