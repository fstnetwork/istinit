mod command;
mod config;
mod error;
mod run;

use self::{command::Command, error::Error};

fn main() -> Result<(), Error> {
    let cmd = Command::new();
    cmd.run()
}
