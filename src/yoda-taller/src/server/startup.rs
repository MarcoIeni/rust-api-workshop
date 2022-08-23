use std::{io, net::TcpListener, sync::Arc, time::Duration};

use anyhow::Context;
use axum::{routing::get, Extension, Router};

use crate::{server::routes, settings::Settings, YodaTaller};

pub struct Application {
    tcp_listener: TcpListener,
    settings: Settings,
}

impl Application {
    pub fn bind(settings: Settings) -> io::Result<Application> {
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
            .route("/health_check", get(routes::health_check))
            .route("/taller/:name", get(routes::taller_than))
            .layer(Extension(yoda_taller));

        axum::Server::from_tcp(self.tcp_listener)
            .context("cannot build server")?
            .serve(app.into_make_service())
            .await
            .context("cannot run server")?;

        Ok(())
    }

    pub fn tcp_listener(&self) -> &TcpListener {
        &self.tcp_listener
    }
}
