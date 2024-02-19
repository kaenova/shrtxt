mod db;
mod env;
mod model;
mod utils;

use model::EmbeddingModel;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Get Environemnt
    // let env = env::get_environment();

    // let pool = db::new_database_pool(
    //     &env.database_host,
    //     &env.database_port,
    //     &env.database_username,
    //     &env.database_password,
    //     &env.database_name,
    // )
    // .await;
    // let dummy_project_id = "clsr02t9i000008jq38rg9he1";
    let dummy_text = vec!["Hello, world!", "Hello, world!"];

    for i in 0..100 {
        println!("i: {}", i);

        // Testing Embedding
        let model = EmbeddingModel::new(SentenceEmbeddingsModelType::AllMiniLmL6V2);
        let data = model.encode(&dummy_text, 32);
        println!("{data:?}");
    }

    /*
        // Test Database
        db::get_tables(&pool) // Get tables
        .await
        .iter()
        .for_each(|table| println!("Table: {}", table));

    let projects = db::get_project_by_id(&pool, dummy_project_id).await;
    println!("projects len {:?}", projects.len());

    let mut trx = pool.begin().await?;
    db::change_project_status_by_id(&mut trx, dummy_project_id, "tolol").await?;

    let res_id = db::insert_text(&mut trx, dummy_project_id, dummy_text, embeddings).await?;
    println!("res_id {:?}", res_id);

    db::delete_all_texts_by_project_id(&mut trx, dummy_project_id).await?;

    trx.commit().await.unwrap();
    */

    Ok(())
}
