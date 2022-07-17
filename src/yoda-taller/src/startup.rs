use axum::{extract::Path, routing::get, Router};

pub struct Application {
    pub port: u16,
}

impl Application {
    pub async fn run(self) -> anyhow::Result<()> {
        // build our application with a single route
        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/taller/:name", get(taller_than));

        // run it with hyper on localhost:3000
        let address = format!("0.0.0.0:{}", self.port);
        axum::Server::bind(&address.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
        Ok(())
    }
}

async fn health_check() {}

async fn taller_than(Path(person_name): Path<String>) {
    dbg!(person_name);
}
