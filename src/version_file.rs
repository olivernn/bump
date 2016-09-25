use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

use error::BumpError;

use semver::Version;

#[derive(Debug)]
pub struct VersionFile {
    path: PathBuf
}

impl VersionFile {
    pub fn new(path: String) -> VersionFile {
        VersionFile{
            path: PathBuf::from(path)
        }
    }

    pub fn read(&self) -> Result<Version, BumpError> {
        let mut version_file = try!(File::open(&self.path));
        let mut buffer = String::new();
        try!(version_file.read_to_string(&mut buffer));
        let version = try!(Version::parse(&buffer));

        Ok(version)
    }

    pub fn write(&self, version: &Version) -> Result<(), BumpError> {
        let mut version_file = try!(File::create(&self.path));
        try!(write!(&mut version_file, "{}", version));
        Ok(())
    }
}

