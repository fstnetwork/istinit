use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("Limit reached while waiting for Istio ready"))]
    WaitForIstioReadyLimitReached,

    #[snafu(display(
        "Failed to kill Istio with API endpoint {}, error: {}",
        pilot_agent_endpoint,
        source
    ))]
    KillIstioWithApi { pilot_agent_endpoint: String, source: hyper::Error, backtrace: Backtrace },

    #[snafu(display("Could not parse Evnoy admin endpoint: {}", pilot_agent_endpoint))]
    ParsePilotAgentEndpoint { pilot_agent_endpoint: String, source: hyper::http::uri::InvalidUri },
}
