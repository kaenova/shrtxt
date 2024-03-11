mod db;
mod env;
mod model;
mod utils;

use core::result::Result::Ok;
use std::{collections::HashSet, sync::Arc};

use model::EmbeddingModel;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType;
use simple_error::SimpleError;
use sqlx::{Pool, Postgres};
use tokio_nsq::{
    NSQChannel, NSQConsumerConfig, NSQConsumerConfigSources, NSQConsumerLookupConfig, NSQEvent,
    NSQProducer, NSQProducerConfig, NSQTopic,
};

async fn handle_message(
    pool: &Pool<Postgres>,
    model: &EmbeddingModel,
    project_id: &str,
    batch_size: &i64,
    nsqd_producer: &mut NSQProducer,
    nsqd_topic_producer: &Arc<NSQTopic>,
) -> Result<(), SimpleError> {
    println!("message body = {}", project_id);

    let mut trx = pool.begin().await.unwrap();

    match db::get_project_by_id(&pool, project_id).await {
        Err(e) => {
            trx.rollback().await.unwrap();
            return Err(SimpleError::new(format!("Error getting project {}", e)));
        }
        Ok(v) => {
            if v.status != "[1/4 Steps] In Queue" {
                trx.rollback().await.unwrap();
                println!("Project {} status is not [1/4 Steps] In Queue", project_id);
                return Ok(());
            }
        }
    }

    match db::change_project_status_by_id(
        &mut trx,
        project_id.to_owned().as_str(),
        "[1/4 Steps] Processing",
    )
    .await
    {
        Err(_) => {
            trx.rollback().await.unwrap();
            return Err(SimpleError::new("Error changing project status"));
        }
        Ok(_) => {}
    }

    let texts = db::get_text_by_project_id(&pool, project_id).await;
    println!("total texts: {}", texts.len());

    if texts.len() == 0 {
        trx.rollback().await.unwrap();
        return Err(SimpleError::new("No texts found"));
    }

    let texts_data: Vec<String> = texts.iter().map(|t| t.value.clone()).collect();
    let model_data: Vec<&str> = texts_data.iter().map(|s| s.as_str()).collect();
    let encoded_data: Vec<Vec<f32>> = model.encode(&model_data, *batch_size);

    if encoded_data.len() != texts.len() {
        trx.rollback().await.unwrap();
        return Err(SimpleError::new("Error encoding texts, data not matched"));
    }

    for (i, value) in texts.iter().enumerate() {
        let embeddings = encoded_data[i].to_vec();
        match db::insert_text_embeddings_by_id(&mut trx, value.id, embeddings).await {
            Err(e) => {
                trx.rollback().await.unwrap();
                return Err(SimpleError::new(format!(
                    "Error inserting text embeddings {}",
                    e
                )));
            }
            Ok(_) => {}
        }
    }

    println!("Embeddings inserted");

    match db::change_project_status_by_id(
        &mut trx,
        project_id.to_owned().as_str(),
        "[2/4 Steps] In Queue",
    )
    .await
    {
        Err(_) => {
            trx.rollback().await.unwrap();
            return Err(SimpleError::new("Error changing project status"));
        }
        Ok(_) => {}
    }

    match nsqd_producer
        .publish(&nsqd_topic_producer, project_id.as_bytes().to_vec())
        .await {
        Err(e) => {
            return Err(SimpleError::new(format!("Error publishing message {}", e)));
        }
        Ok(_) => {
            assert!(matches!(
                nsqd_producer.consume().await.unwrap(),
                NSQEvent::Ok()
            ));
        }
    }

    match trx.commit().await {
        Err(e) => {
            return Err(SimpleError::new(format!(
                "Error committing transaction {}",
                e
            )));
        }
        Ok(_) => {}
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Constants
    let batch_size = 32;

    // Get Environemnt
    let env = env::get_environment();

    // Prepare Database
    let pool = db::new_database_pool(
        &env.database_host,
        &env.database_port,
        &env.database_username,
        &env.database_password,
        &env.database_name,
    )
    .await;

    // Prepare Embedding
    let model = EmbeddingModel::new(SentenceEmbeddingsModelType::AllMiniLmL6V2);

    // Prepare NSQD Consumer
    let topic_consumer = NSQTopic::new(env.nsqd_embed_text_topic).unwrap();
    let channel_consumer = NSQChannel::new(env.nsqd_embed_text_channel).unwrap();
    let mut addresses = HashSet::new();
    addresses.insert(
        format!(
            "http://{}:{}",
            env.nsqd_consumer_host, env.nsqd_consumer_port
        )
        .to_string(),
    );

    // Prepare NSQD Producer
    let topic_producer = NSQTopic::new(env.nsqd_cluster_text_topic).unwrap();
    let mut producer = NSQProducerConfig::new(format!(
        "{}:{}",
        env.nsqd_producer_host, env.nsqd_producer_port
    ))
    .build();
    assert!(matches!(
        producer.consume().await.unwrap(),
        NSQEvent::Healthy()
    ));
    println!("Connected to NSQD Producer");

    let mut consumer = NSQConsumerConfig::new(topic_consumer, channel_consumer)
        .set_max_in_flight(15)
        .set_sources(NSQConsumerConfigSources::Lookup(
            NSQConsumerLookupConfig::new().set_addresses(addresses),
        ))
        .build();

    println!("Starting loop to consume messages...");

    loop {
        let message = consumer.consume_filtered().await.unwrap();
        let project_id = std::str::from_utf8(&message.body).unwrap();

        match handle_message(
            &pool,
            &model,
            project_id,
            &batch_size,
            & mut producer,
            &topic_producer,
        )
        .await
        {
            Ok(_) => {
                message.finish().await;
            }
            Err(e) => {
                message
                    .requeue(tokio_nsq::NSQRequeueDelay::DefaultDelay)
                    .await;
                println!("Requeueing with Error: {}", e);
            }
        }
    }
}
