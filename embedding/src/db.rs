use serde_json::json;
use sqlx::{Pool, Postgres, Row, Transaction};

// Table Name "Project"
pub struct Project {
    pub id: String,   // Column names "id"
    pub name: String, // Column names "name"
}

pub async fn new_database_pool(
    host: &str,
    port: &str,
    username: &str,
    password: &str,
    database: &str,
) -> Pool<Postgres> {
    let str_conn = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, database
    );

    let pool = Pool::<Postgres>::connect(&str_conn).await.unwrap();

    let q_res = sqlx::query("SELECT 1 as one").fetch_all(&pool).await;
    match q_res {
        Err(e) => panic!("Cannot connect to database: {}", e),
        Ok(_) => {
            println!("Connected to database");
        }
    }
    pool
}

pub async fn get_tables(pool: &Pool<Postgres>) -> Vec<String> {
    let q_res = sqlx::query(
        r#"SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';"#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let mut tables = Vec::new();

    for row in q_res {
        let table_name: String = row.get("table_name");
        tables.push(table_name);
    }
    tables
}

pub async fn get_project_by_id(pool: &Pool<Postgres>, id: &str) -> Vec<Project> {
    let q_res = sqlx::query(r#"SELECT * FROM "Project" WHERE id = $1"#)
        .bind(id)
        .fetch_all(pool)
        .await
        .unwrap();

    let mut projects = Vec::new();

    for row in q_res {
        let id: String = row.get("id");
        let name: String = row.get("name");
        projects.push(Project { id, name });
    }
    projects
}

pub async fn change_project_status_by_id(
    trx: &mut Transaction<'_, Postgres>,
    id: &str,
    status: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(r#"UPDATE "Project" SET status = $1 WHERE id = $2 RETURNING id"#)
        .bind(status)
        .bind(id)
        .execute(&mut **trx)
        .await;

    match res {
        Ok(v) => Ok(v.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn delete_all_texts_by_project_id(
    trx: &mut Transaction<'_, Postgres>,
    project_id: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(r#"DELETE FROM "Text" WHERE "projectId" = $1"#)
        .bind(project_id)
        .execute(&mut **trx)
        .await;

    match res {
        Ok(v) => Ok(v.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn insert_text(
    trx: &mut Transaction<'_, Postgres>,
    project_id: &str,
    value: &str,
    embeddings: Vec<f32>,
) -> Result<u64, sqlx::Error> {
    let embeddings_json = json!(embeddings);

    let res = sqlx::query(
        r#"INSERT INTO "Text" ("projectId", value, embeddings) VALUES ($1, $2, $3) RETURNING id"#,
    )
    .bind(project_id)
    .bind(value)
    .bind(embeddings_json)
    .execute(&mut **trx)
    .await;

    match res {
        Ok(v) => Ok(v.rows_affected()),
        Err(e) => Err(e),
    }
}
