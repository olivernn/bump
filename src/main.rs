extern crate semver;
extern crate rustc_serialize;
extern crate docopt;

pub mod command;
pub mod version_increment;
pub mod version_file;
pub mod error;

use command::{Command, Args};
use version_increment::VersionIncrement;
use version_file::VersionFile;
use error::BumpError;

use docopt::Docopt;
use semver::{Version, Identifier};

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
