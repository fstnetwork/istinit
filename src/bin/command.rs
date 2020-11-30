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

    #[structopt(long = "with-istio", env = "WITH_ISTIO", parse(from_str = parse_bool_str))]
    with_istio: Option<bool>,

    #[structopt(
        long = "pilot-agent-endpoint",
        env = "PILOT_AGENT_ENDPOINT",
        default_value = "http://127.0.0.1:15020"
    )]
    pilot_agent_endpoint: String,

    #[structopt(long = "kill-istio", env = "KILL_ISTIO", parse(from_str = parse_bool_str))]
    kill_istio: Option<bool>,

    command: String,

    args: Vec<String>,
}

impl Command {
    #[inline]
    pub fn new() -> Command { Command::from_args() }

    pub fn run(self) -> Result<i32, Error> { run::run(self.into()) }
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

        let istio = if let Some(true) = with_istio {
            Some(config::Istio { pilot_agent_endpoint, kill_istio: kill_istio.unwrap_or(false) })
        } else {
            None
        };

        Config { enable_process_subreaper, process, istio }
    }
}

fn parse_bool_str(s: &str) -> bool { !matches!(s, "" | "false") }

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_bool_str() {
        use super::parse_bool_str;

        assert_eq!(parse_bool_str(""), false);
        assert_eq!(parse_bool_str("false"), false);
        assert_eq!(parse_bool_str("true"), true);
        assert_eq!(parse_bool_str("any"), true);
    }
}
