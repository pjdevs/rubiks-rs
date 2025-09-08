#[tokio::main]
async fn main() {
    rubiks_api::database::migration::ensure_db().await;
}