use std::{
    env,
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};

fn main() -> Result<(), Error> {

    copy_memory_config()?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

/// Make `memory.x` available to dependent crates
fn copy_memory_config() -> Result<(), Error> {
    let memory_x = include_bytes!("linker.x").as_ref();

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);

    File::create(out_dir.join("memory.x"))?.write_all(memory_x)?;

    // Tell Cargo where to find the file.
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=linker.x");

    Ok(())
}

#[derive(Debug)]
enum Error {
    Env(env::VarError),
    Io(io::Error),
}

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Self::Env(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}