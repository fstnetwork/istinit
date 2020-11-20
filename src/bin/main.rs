mod command;
mod error;

use self::{command::Command, error::Error};

fn main() -> Result<(), Error> {
    let cmd = Command::new();
    cmd.run()
}
