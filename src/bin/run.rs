use std::{path::PathBuf, time::Duration};

use snafu::ResultExt;
use tokio::runtime::Runtime;
use tokio_compat_02::FutureExt;

use istinit::istio;

use crate::{
    config::Config,
    error::{self, Error},
};

pub fn run(config: Config) -> Result<(), Error> {
    {
        use tracing_subscriber::prelude::*;

        let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
        let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
            .unwrap();

        tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
    }

    let runtime = Runtime::new().context(error::InitializeTokioRuntime)?;
    runtime.block_on(
        async {
            if let Some(istio) = &config.istio {
                tracing::info!("Wait for Envoy ready");
                let retry_interval = Duration::from_secs(3);
                istio::wait_for_envoy_ready(&istio.pilot_agent_endpoint, retry_interval, None)
                    .await
                    .context(error::WaitForEnvoyReady)?;
            }

            {
                let process = config.process;
                tracing::info!("Spawn process {} and wait", process.command);
                if let Err(err) = spawn_and_wait_executable(&process.command, &process.args).await {
                    tracing::warn!("Error: {}", err);
                };
            }

            if let Some(istio) = &config.istio {
                if istio.kill_istio {
                    tracing::info!("Kill Istio");
                    istio::kill_istio_with_api(&istio.pilot_agent_endpoint)
                        .await
                        .context(error::KillIstio)?;
                }
            }

            Ok(())
        }
        .compat(),
    )?;

    Ok(())
}

async fn spawn_and_wait_executable(command: &str, args: &[String]) -> Result<i32, Error> {
    let mut child = tokio::process::Command::new(command)
        .args(args)
        .spawn()
        .context(error::SpawnProcess { executable_path: PathBuf::from(command) })?;

    let _status = child.wait().await.context(error::WaitForChildProcess)?;

    Ok(0)
}
