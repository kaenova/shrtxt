mod db;
mod env;
mod model;

use model::EmbeddingModel;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Get Environemnt
    let env = env::get_environment();

    // Testing Embedding
    let model = EmbeddingModel::new(SentenceEmbeddingsModelType::AllMiniLmL6V2);
    let test = "Anjing kau";
    let data = model.encode(test);
    println!("{data:?}");

    let pool =  db::new_database_pool(
        &env.database_host,
        &env.database_port,
        &env.database_username,
        &env.database_password,
        &env.database_name,
    )
    .await;

    db::get_tables(&pool) // Get tables
        .await
        .iter()
        .for_each(|table| println!("Table: {}", table));

    let projects = db::get_project_by_id(&pool, "clsr02t9i000008jq38rg9he1").await;
    println!("projects len {:?}", projects.len());

    let mut trx = pool.begin().await?;
    db::change_project_status_by_id(&mut trx, "clsr02t9i000008jq38rg9he1", "tolol").await?;

    trx.rollback().await.unwrap();

    Ok(())
}
