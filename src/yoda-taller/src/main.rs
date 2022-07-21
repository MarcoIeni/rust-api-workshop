use yoda_taller::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let application = Application { port: 3000 };
    application.bind()?.run().await
}
