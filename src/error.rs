use std::error;
use std::fmt;
use std::io;
use std::process;

use semver::SemVerError;

#[derive(Debug)]
pub enum BumpError {
    Io(io::Error),
    SemVer(SemVerError)
}

impl BumpError {
    pub fn exit(&self) -> ! {
        println!("{}", self);
        process::exit(1);
    }
}

impl fmt::Display for BumpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BumpError::Io(ref e) => write!(f, "IO Error: {}", e),
            BumpError::SemVer(ref e) => write!(f, "SemVer Error: {}", e),
        }
    }
}

impl error::Error for BumpError {
    fn description(&self) -> &str {
        match *self {
            BumpError::Io(ref e) => e.description(),
            BumpError::SemVer(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            BumpError::Io(ref e) => Some(e),
            BumpError::SemVer(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for BumpError {
    fn from(e: io::Error) -> BumpError {
        BumpError::Io(e)
    }
}

impl From<SemVerError> for BumpError {
    fn from(e: SemVerError) -> BumpError {
        BumpError::SemVer(e)
    }
}

