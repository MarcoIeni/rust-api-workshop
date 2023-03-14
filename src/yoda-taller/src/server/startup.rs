use {
    super::shutdown::shutdown_handler,
    crate::{server::taller_route, settings::Settings},
    anyhow::Context,
    axum::{routing::get, Router},
    axum_tracing_opentelemetry::opentelemetry_tracing_layer,
    std::{
        io,
        net::{SocketAddr, TcpListener},
        sync::Arc,
    },
};

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
        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/taller/:name", get(taller_route::taller_than))
            .with_state(yoda_taller)
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

/// Endpoint that returns 200. Used to check if the server is up.
pub async fn health_check() {}
