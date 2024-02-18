use std::env;

pub struct Environment {
  pub database_host: String,
  pub database_port: String,
  pub database_username: String,
  pub database_password: String,
  pub database_name: String,
}

pub fn get_environment() -> Environment {
  let database_host = env::var("DB_HOST").expect("DB_HOST must be set");
  let database_port = env::var("DB_PORT").expect("DB_PORT must be set");
  let database_username = env::var("DB_USER").expect("DB_USER must be set");
  let database_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
  let database_name = env::var("DB_NAME").expect("DB_NAME must be set");

  Environment {
    database_host: database_host,
    database_port: database_port,
    database_username: database_username,
    database_password: database_password,
    database_name: database_name,
  }
}