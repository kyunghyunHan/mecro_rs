use mecro_rs::{google_search, letskorail_serch};
#[tokio::main]
async fn main() {
    letskorail_serch::example().await.unwrap();
}
