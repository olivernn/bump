extern crate semver;
extern crate rustc_serialize;
extern crate docopt;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::fmt;
use std::error;

use docopt::Docopt;
use semver::{Version, Identifier, SemVerError};

const USAGE: &'static str = "
bump

Usage:
    bump init
    bump major
    bump minor
    bump patch
    bump build <build>
    bump pre <pre>
    bump

Options:
    -h, --help           Show this screen
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
    Init,
    Print,
    Bump(VersionIncrement),
}

impl From<Args> for Command {
    fn from(args: Args) -> Command {
        if args.cmd_init {
            return Command::Init
        }

        if args.cmd_major {
            return Command::Bump(VersionIncrement::Major);
        }

        if args.cmd_minor {
            return Command::Bump(VersionIncrement::Minor);
        }

        if args.cmd_patch {
            return Command::Bump(VersionIncrement::Patch);
        }

        if args.cmd_pre {
            return Command::Bump(VersionIncrement::Pre(args.arg_pre));
        }

        if args.cmd_build {
            return Command::Bump(VersionIncrement::Build(args.arg_build));
        }

        return Command::Print
    }
}

#[derive(Debug)]
enum BumpError {
    Io(io::Error),
    SemVer(SemVerError)
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

fn write_version(version: &Version) -> Result<(), BumpError> {
    let mut version_file = try!(File::create("VERSION"));
    try!(write!(&mut version_file, "{}", version));
    Ok(())
}

fn read_version() -> Result<Version, BumpError> {
    let mut version_file = try!(File::open("VERSION"));
    let mut buffer = String::new();
    try!(version_file.read_to_string(&mut buffer));
    let version = try!(Version::parse(&buffer));

    Ok(version)
}

fn print() -> Result<(), BumpError> {
    let version = try!(read_version());
    println!("{}", version);
    return Ok(());
}

fn init() -> Result<(), BumpError> {
    let version = try!(Version::parse("0.0.0"));
    try!(write_version(&version));
    return Ok(());
}

fn bump(action: VersionIncrement) -> Result<(), BumpError> {
    let mut version = try!(read_version());

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

    try!(write_version(&version));
    return Ok(());
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let result = match Command::from(args) {
        Command::Print => print(),
        Command::Init => init(),
        Command::Bump(increment) => bump(increment)
    };

    result.unwrap_or_else(|e| println!("{}", e));
}
