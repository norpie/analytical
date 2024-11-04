use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RawArgs {
    #[clap(long)]
    db_path: Option<PathBuf>,
}

impl RawArgs {
    fn populate(mut self) -> Self {
        self.db_path = Some(PathBuf::from("/etc/storeful/default.db"));
        self
    }
}

impl From<RawArgs> for Args {
    fn from(raw_args: RawArgs) -> Self {
        Args {
            db_path: raw_args.db_path.unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Args {
    db_path: PathBuf,
}

impl Default for Args {
    fn default() -> Self {
        RawArgs::parse().populate().into()
    }
}

impl Args {
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
