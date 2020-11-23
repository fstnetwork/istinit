use std::{thread, time::Duration};

use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use tokio_compat_02::FutureExt;

use istinit::istio;

pub struct MockIstio {
    start_signal: Option<std::sync::mpsc::Sender<()>>,
    srv_rx: tokio::sync::mpsc::UnboundedReceiver<actix_web::dev::Server>,
}

impl MockIstio {
    pub fn new(host: String, port: u16) -> MockIstio {
        let (srv_tx, srv_rx) = tokio::sync::mpsc::unbounded_channel();
        let (start_signal, start_rx) = std::sync::mpsc::channel();

        // start server in another thread
        thread::spawn(move || {
            start_rx.recv().unwrap();

            let sys = System::new("mock-istio");

            let srv = HttpServer::new(|| {
                App::new()
                    .route("/healthz/ready", web::get().to(|| HttpResponse::Ok()))
                    .route("/quitquitquit", web::post().to(|| HttpResponse::Ok()))
            })
            .bind(format!("{}:{}", host, port))?
            .run();

            let _ = srv_tx.send(srv);
            sys.run()
        });

        MockIstio { srv_rx, start_signal: Some(start_signal) }
    }

    pub async fn start(&mut self, init_delay: Option<Duration>) {
        if let Some(start_signal) = std::mem::replace(&mut self.start_signal, None) {
            tokio::spawn(async move {
                if let Some(delay) = init_delay {
                    tokio::time::sleep(delay).await;
                }
                start_signal.send(()).unwrap();
            });
        }
    }

    pub async fn stop(mut self) {
        let srv = self.srv_rx.recv().await.unwrap();
        srv.stop(true).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_istio() -> Result<(), Box<dyn std::error::Error>> {
    {
        use tracing_subscriber::prelude::*;

        let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
        let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
            .unwrap();

        tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
    }

    let pilot_host = "127.0.0.1";
    let pilot_port: u16 = 63726;
    let pilot_agent_endpoint = format!("http://{}:{}", pilot_host, pilot_port);

    async {
        let retry_interval = Duration::from_millis(100);

        // start server in another thread
        let mut mock_istio = MockIstio::new(pilot_host.to_owned(), pilot_port);
        mock_istio.start(Some(Duration::from_secs(1))).await;

        istio::wait_for_envoy_ready(&pilot_agent_endpoint, retry_interval, None).await.unwrap();

        istio::kill_istio_with_api(&pilot_agent_endpoint).await.unwrap();

        // stop server
        mock_istio.stop().await;
    }
    .compat()
    .await;

    Ok(())
}
