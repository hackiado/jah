use crate::archive::Archive;
use std::process::ExitCode;
#[doc = "To create archive"]
pub mod archive;

fn main() -> ExitCode {
    Archive::new().create()
}
