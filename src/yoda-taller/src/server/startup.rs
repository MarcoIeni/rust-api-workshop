use std::{
    io,
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use anyhow::Context;
use axum::{routing::get, Extension, Router};
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;

use crate::{server::routes, settings::Settings};

use super::shutdown::shutdown_handler;

pub struct Application {
    tcp_listener: TcpListener,
    settings: Settings,
}

impl Application {
    pub fn bind(settings: Settings) -> io::Result<Self> {
        let socket_addr = SocketAddr::from(([0, 0, 0, 0], settings.application.port));
        let tcp_listener = TcpListener::bind(socket_addr)?;
        Ok(Self {
            tcp_listener,
            settings,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let yoda_taller = {
            let yoda_taller = self.settings.swapi.yoda_taller()?;
            Arc::new(yoda_taller)
        };
        // build our application with a single route
        let app = Router::new()
            .route("/health_check", get(routes::health_check))
            .route("/taller/:name", get(routes::taller_than))
            .layer(Extension(yoda_taller))
            .layer(opentelemetry_tracing_layer());

        axum::Server::from_tcp(self.tcp_listener)
            .context("cannot build server")?
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_handler())
            .await
            .context("cannot run server")?;

        Ok(())
    }

    pub fn tcp_listener(&self) -> &TcpListener {
        &self.tcp_listener
    }
}
