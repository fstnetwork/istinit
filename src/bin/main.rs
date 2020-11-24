mod command;
mod config;
mod error;
mod run;

use self::{command::Command, error::Error};

fn main() -> Result<(), Error> {
    let cmd = Command::new();
    let exit_code = cmd.run()?;
    std::process::exit(exit_code);
}
