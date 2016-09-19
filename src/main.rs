extern crate semver;
extern crate rustc_serialize;
extern crate docopt;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::fmt;
use std::error;
use std::path::PathBuf;

use docopt::Docopt;
use semver::{Version, Identifier, SemVerError};

const USAGE: &'static str = "
bump

Usage:
    bump init [options]
    bump major [options]
    bump minor [options]
    bump patch [options]
    bump build <build> [options]
    bump pre <pre> [options]
    bump [options]

Options:
    -h, --help           Show this screen
    -f, --file=<file>    Specify the version file path
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_init: bool,
    cmd_major: bool,
    cmd_minor: bool,
    cmd_patch: bool,
    cmd_pre: bool,
    arg_pre: String,
    cmd_build: bool,
    arg_build: String,
    flag_file: Option<String>,
}

#[derive(Debug)]
struct VersionFile {
    path: PathBuf
}

impl VersionFile {
    fn new(path: String) -> VersionFile {
        VersionFile{
            path: PathBuf::from(path)
        }
    }

    fn read(&self) -> Result<Version, BumpError> {
        let mut version_file = try!(File::open(&self.path));
        let mut buffer = String::new();
        try!(version_file.read_to_string(&mut buffer));
        let version = try!(Version::parse(&buffer));

        Ok(version)
    }

    fn write(&self, version: &Version) -> Result<(), BumpError> {
        let mut version_file = try!(File::create(&self.path));
        try!(write!(&mut version_file, "{}", version));
        Ok(())
    }
}

#[derive(Debug)]
enum VersionIncrement {
    Major,
    Minor,
    Patch,
    Pre(String),
    Build(String),
}

#[derive(Debug)]
enum Command {
    Init(VersionFile),
    Print(VersionFile),
    Bump(VersionIncrement, VersionFile),
}

impl From<Args> for Command {
    fn from(args: Args) -> Command {
        let version_path = args.flag_file.unwrap_or("VERSION".to_owned());
        let version_file = VersionFile::new(version_path);

        if args.cmd_init {
            return Command::Init(version_file)
        }

        if args.cmd_major {
            return Command::Bump(VersionIncrement::Major, version_file);
        }

        if args.cmd_minor {
            return Command::Bump(VersionIncrement::Minor, version_file);
        }

        if args.cmd_patch {
            return Command::Bump(VersionIncrement::Patch, version_file);
        }

        if args.cmd_pre {
            return Command::Bump(VersionIncrement::Pre(args.arg_pre), version_file);
        }

        if args.cmd_build {
            return Command::Bump(VersionIncrement::Build(args.arg_build), version_file);
        }

        return Command::Print(version_file)
    }
}

#[derive(Debug)]
enum BumpError {
    Io(io::Error),
    SemVer(SemVerError)
}

impl BumpError {
    fn exit(&self) -> ! {
        println!("{}", self);
        std::process::exit(1);
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

fn print(version_file: VersionFile) -> Result<(), BumpError> {
    let version = try!(version_file.read());
    println!("{}", version);
    return Ok(());
}

fn init(version_file: VersionFile) -> Result<(), BumpError> {
    let version = try!(Version::parse("0.0.0"));
    try!(version_file.write(&version));
    return Ok(());
}

fn bump(action: VersionIncrement, version_file: VersionFile) -> Result<(), BumpError> {
    let mut version = try!(version_file.read());

    match action {
        VersionIncrement::Major => version.increment_major(),
        VersionIncrement::Minor => version.increment_minor(),
        VersionIncrement::Patch => version.increment_patch(),
        VersionIncrement::Pre(pre) => {
            version.pre.clear();
            version.build.clear();
            version.pre.push(Identifier::AlphaNumeric(pre))
        },
        VersionIncrement::Build(build) => {
            version.build.clear();
            version.build.push(Identifier::AlphaNumeric(build))
        },
    }

    try!(version_file.write(&version));
    println!("{}", version);
    return Ok(());
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let result = match Command::from(args) {
        Command::Print(version_file) => print(version_file),
        Command::Init(version_file) => init(version_file),
        Command::Bump(increment, version_file) => bump(increment, version_file)
    };

    result.unwrap_or_else(|e| e.exit());
}
