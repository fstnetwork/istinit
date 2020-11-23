use std::time::Duration;

use snafu::ResultExt;

use crate::error::{self, Error};

/// # Errors
///
/// Will return `Err` if failed to parse pilot agent endpoint or retry limit
/// reached
pub async fn wait_for_envoy_ready(
    endpoint: &str,
    interval: Duration,
    max_retry_limit: Option<usize>,
) -> Result<(), Error> {
    let client = hyper::Client::new();
    let uri: hyper::Uri = format!("{}/healthz/ready", endpoint).parse().with_context(|| {
        error::ParsePilotAgentEndpoint { pilot_agent_endpoint: endpoint.to_owned() }
    })?;

    let mut retry_count = 0;
    loop {
        retry_count += 1;
        match client.get(uri.clone()).await {
            Ok(resp) => match resp.status() {
                hyper::StatusCode::OK => return Ok(()),
                status => {
                    tracing::info!("Polling Envoy ({}), status : {}", retry_count, status);
                }
            },
            Err(source) => {
                tracing::info!("Polling Envoy ({}), error: {}", retry_count, source);
            }
        }

        match max_retry_limit {
            Some(retry_limit) if retry_count >= retry_limit => {
                return Err(Error::WaitForIstioReadyLimitReached);
            }
            _ => {}
        }

        tokio::time::sleep(interval).await;
    }
}

/// # Errors
///
/// Will return `Err` if failed to parse pilot agent endpoint or failed to kill
/// istio
pub async fn kill_istio_with_api(endpoint: &str) -> Result<(), Error> {
    tracing::info!("Stopping Istio using Istio API '{}'", endpoint);

    let client = hyper::Client::new();
    let request = {
        let uri: hyper::Uri = format!("{}/quitquitquit", endpoint).parse().with_context(|| {
            error::ParsePilotAgentEndpoint { pilot_agent_endpoint: endpoint.to_owned() }
        })?;

        hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(hyper::Body::empty())
            .expect("Building request always succeed")
    };

    client
        .request(request)
        .await
        .with_context(|| error::KillIstioWithApi { pilot_agent_endpoint: endpoint.to_owned() })?;

    Ok(())
}
