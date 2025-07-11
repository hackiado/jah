use crate::archive::Archive;
use std::path::Path;
use std::process::ExitCode;
#[doc = "To create archive"]
pub mod archive;

fn main() -> ExitCode {
    if Path::new("Cargo.toml").exists() {
        return Archive::new().create();
    }
    println!("No Cargo.toml found");
    ExitCode::FAILURE
}
