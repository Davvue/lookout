use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    name: String
}
impl Args {
    pub fn name(&self) -> &str {
        &self.name
    }
}