use yoda_taller::startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    startup::run().await
}
