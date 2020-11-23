use structopt::StructOpt;

use crate::{
    config::{self, Config},
    error::Error,
    run,
};

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(long = "enable-process-subreaper", short = "s", env = "ENABLE_PROCESS_SUBREAPER")]
    enable_process_subreaper: bool,

    #[structopt(long = "with-istio", env = "WITH_ISTIO")]
    with_istio: bool,

    #[structopt(
        long = "pilot-agent-endpoint",
        env = "PILOT_AGENT_ENDPOINT",
        default_value = "http://127.0.0.1:15021"
    )]
    pilot_agent_endpoint: String,

    #[structopt(long = "kill-istio", env = "KILL_ISTIO")]
    kill_istio: bool,

    command: String,

    args: Vec<String>,
}

impl Command {
    #[inline]
    pub fn new() -> Command { Command::from_args() }

    pub fn run(self) -> Result<(), Error> { run::run(self.into()) }
}

impl Into<Config> for Command {
    fn into(self) -> Config {
        let Command {
            enable_process_subreaper,
            with_istio,
            pilot_agent_endpoint,
            kill_istio,
            command,
            args,
        } = self;

        let process = config::Process { command, args };

        let istio = if with_istio {
            Some(config::Istio { pilot_agent_endpoint, kill_istio })
        } else {
            None
        };

        Config { enable_process_subreaper, process, istio }
    }
}
