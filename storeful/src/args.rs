use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RawArgs {
    #[clap(long)]
    db_path: Option<PathBuf>,

    #[clap(long)]
    host: Option<String>,

    #[clap(long)]
    port: Option<u16>,

    #[clap(long)]
    http: bool,
}

impl RawArgs {
    fn populate(mut self) -> Self {
        self.db_path = Some(PathBuf::from("./default.db"));
        self
    }
}

impl From<RawArgs> for Args {
    fn from(raw_args: RawArgs) -> Self {
        Args {
            db_path: raw_args.db_path.unwrap(),
            host: "127.0.0.1".to_string(),
            port: 4040,
            http: raw_args.http,
        }
    }
}

#[derive(Debug)]
pub struct Args {
    pub db_path: PathBuf,
    pub host: String,
    pub port: u16,
    pub http: bool,
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

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn http(&self) -> bool {
        self.http
    }
}
