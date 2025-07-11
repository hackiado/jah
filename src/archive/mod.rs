use std::fs;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;
use std::process::{Command, ExitCode};

const CHECKUP: [&str; 6] = [
    "fmt --check",
    "test --no-fail-fast",
    "clippy -- -D clippy::all -D clippy::cargo",
    "audit",
    "clean",
    "build --release",
];
#[derive(Clone)]
#[doc = include_str!("../../docs/archive.md")]
pub struct Archive {
    app: String,
}

impl Default for Archive {
    fn default() -> Self {
        Self {
            app: env!("CARGO_PKG_NAME").to_string(),
        }
    }
}
impl Archive {
    #[must_use]
    #[doc = "Alias to the default instance"]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc = "create archive after tests"]
    pub fn create(self) -> ExitCode {
        if Path::new("Cargo.toml").exists() {
            assert!(self.clone().check(), "Cannot create archive");
            if Path::new(".jah").is_dir() {
                assert!(
                    remove_dir_all(".jah").is_ok(),
                    "Cannot remove .jah directory"
                );
            }
            assert!(
                create_dir_all(".jah").is_ok(),
                "Cannot create .jah directory"
            );
            assert!(
                create_dir_all(".jah/bin").is_ok(),
                "Cannot create .jah directory"
            );
            assert!(
                fs::copy(
                    format!("target/release/{}", self.clone().app),
                    format!(".jah/bin/{}", self.clone().app),
                )
                .is_ok(),
                "Could not copy .jah directory"
            );
            return ExitCode::SUCCESS;
        }
        println!("No Cargo.toml file found");
        ExitCode::FAILURE
    }

    #[doc = "test application"]
    fn check(self) -> bool {
        for checkup in &CHECKUP {
            if Command::new("cargo")
                .args(checkup.split_whitespace())
                .current_dir(".")
                .spawn()
                .expect("please install cargo")
                .wait()
                .expect("Failed to wait on child")
                .success()
            {
                continue;
            }
            return false;
        }
        true
    }
}
