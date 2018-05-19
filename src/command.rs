use version_increment::VersionIncrement;
use version_file::VersionFile;

#[derive(Debug, Deserialize)]
pub struct Args {
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
pub enum Command {
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

