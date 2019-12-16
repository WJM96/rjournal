use fern;
use gio::prelude::*;
use log::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};

macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn load_file(file: &gio::File) -> std::io::Result<String> {
    if let Some(path) = file.get_path() {
        let read_file = File::open(path);
        match read_file {
            Ok(read_file) => {
                let mut reader = BufReader::new(read_file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);
                Ok(contents)
            }
            Err(err) => Err(err),
        }
    } else {
        error!("Couldn't get filename!");
        Err(Error::new(
            ErrorKind::Other,
            "The gio::File did not have a path",
        ))
    }
}
