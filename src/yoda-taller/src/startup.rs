use std::{net::TcpListener, sync::Arc, time::Duration};

use anyhow::Context;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use serde::Serialize;

use crate::{settings::Settings, YodaTaller};

pub struct Application {
    tcp_listener: TcpListener,
    settings: Settings,
}

impl Application {
    pub fn build(settings: Settings) -> anyhow::Result<Application> {
        let address = format!("0.0.0.0:{}", settings.application.port);
        let tcp_listener = TcpListener::bind(&address)?;
        Ok(Application {
            tcp_listener,
            settings,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let yoda_taller = {
            let timeout_duration = Duration::from_millis(self.settings.swapi.timeout_milliseconds);
            let yoda_taller = YodaTaller::new(self.settings.swapi.base_url, timeout_duration);
            Arc::new(yoda_taller)
        };
        // build our application with a single route
        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/taller/:name", get(taller_than))
            .layer(Extension(yoda_taller));

        axum::Server::from_tcp(self.tcp_listener)
            .context("cannot build server")?
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }

    pub fn tcp_listener(&self) -> &TcpListener {
        &self.tcp_listener
    }
}

async fn health_check() {}

async fn taller_than(
    Path(person_name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Json<YodaTallerResponse> {
    let taller = yoda_taller.is_taller_than(&person_name).await.unwrap();
    YodaTallerResponse { taller }.into()
}

#[derive(Debug, Serialize)]
struct YodaTallerResponse {
    taller: bool,
}
