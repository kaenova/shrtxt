use std::env;

pub struct Environment {
    pub database_host: String,
    pub database_port: String,
    pub database_username: String,
    pub database_password: String,
    pub database_name: String,
    pub nsqd_consumer_host: String,
    pub nsqd_consumer_port: String,
    pub nsqd_producer_host: String,
    pub nsqd_producer_port: String,
    pub nsqd_embed_text_topic: String,
    pub nsqd_embed_text_channel: String,
    pub nsqd_cluster_text_topic: String,
}

pub fn get_environment() -> Environment {
    let database_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let database_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let database_username = env::var("DB_USER").expect("DB_USER must be set");
    let database_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let database_name = env::var("DB_NAME").expect("DB_NAME must be set");

    let nsqd_consumer_host = env::var("NSQD_CONSUMER_HOST").expect("NSQD_HOST must be set");
    let nsqd_consumer_port = env::var("NSQD_CONSUMER_PORT").expect("NSQD_PORT must be set");
    let nsqd_producer_host = env::var("NSQD_PRODUCER_HOST").expect("NSQD_HOST must be set");
    let nsqd_producer_port = env::var("NSQD_PRODUCER_PORT").expect("NSQD_PORT must be set");
    let nsqd_embed_text_topic =
        env::var("NSQD_EMBED_TEXT_TOPIC").expect("NSQD_EMBED_TEXT_TOPIC must be set");
    let nsqd_embed_text_channel =
        env::var("NSQD_EMBED_TEXT_CHANNEL").expect("NSQD_EMBED_TEXT_CHANNEL must be set");
    let nsqd_cluster_text_topic =
        env::var("NSQD_CLUSTER_TEXT_TOPIC").expect("NSQD_CLUSTER_TEXT_TOPIC must be set");

    Environment {
        database_host: database_host,
        database_port: database_port,
        database_username: database_username,
        database_password: database_password,
        database_name: database_name,
        nsqd_consumer_host: nsqd_consumer_host,
        nsqd_consumer_port: nsqd_consumer_port,
        nsqd_producer_host: nsqd_producer_host,
        nsqd_producer_port: nsqd_producer_port,
        nsqd_embed_text_topic: nsqd_embed_text_topic,
        nsqd_embed_text_channel: nsqd_embed_text_channel,
        nsqd_cluster_text_topic: nsqd_cluster_text_topic,
    }
}
