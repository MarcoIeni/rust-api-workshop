use std::net::TcpListener;

use anyhow::Context;
use axum::{extract::Path, routing::get, Router};

pub struct Application {
    pub port: u16,
}

impl Application {
    pub fn bind(&self) -> anyhow::Result<ApplicationBind> {
        let address = format!("0.0.0.0:{}", self.port);
        let tcp_listener = TcpListener::bind(&address)?;
        Ok(ApplicationBind { tcp_listener })
    }
}

pub struct ApplicationBind {
    tcp_listener: TcpListener,
}

impl ApplicationBind {
    pub async fn run(self) -> anyhow::Result<()> {
        // build our application with a single route
        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/taller/:name", get(taller_than));

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

async fn taller_than(Path(person_name): Path<String>) {
    dbg!(person_name);
    // TODO make request
}
