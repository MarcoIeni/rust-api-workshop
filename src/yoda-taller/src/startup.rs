use axum::{extract::Path, routing::get, Router};

pub async fn run() -> anyhow::Result<()> {
    // build our application with a single route
    let app = Router::new().route("/taller/:name", get(taller_than));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn taller_than(Path(person_name): Path<String>) {
    dbg!(person_name);
}
