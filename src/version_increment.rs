#[derive(Debug)]
pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
    Pre(String),
    Build(String),
}

