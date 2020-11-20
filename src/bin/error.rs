use std::path::PathBuf;

use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("Initialize Tokio runtime, error: {}", source))]
    InitializeTokioRuntime { source: std::io::Error, backtrace: Backtrace },

    #[snafu(display("Error occurred while waiting for Envoy ready, error: {}", source))]
    WaitForEnvoyReady { source: istinit::error::Error, backtrace: Backtrace },

    #[snafu(display("Error occurred while killing Istio, error: {}", source))]
    KillIstio { source: istinit::error::Error, backtrace: Backtrace },

    #[snafu(display(
        "Could not spawn new process with executable {}, error: {}",
        executable_path.display(),
        source
    ))]
    SpawnProcess { executable_path: PathBuf, source: std::io::Error, backtrace: Backtrace },

    #[snafu(display("Error occurred while waiting for child process, error: {}", source))]
    WaitForChildProcess { source: std::io::Error, backtrace: Backtrace },
}
