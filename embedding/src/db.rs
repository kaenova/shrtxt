use serde_json::json;
use simple_error::SimpleError;
use sqlx::{Pool, Postgres, Row, Transaction};

// Table Name "Project"
pub struct Project {
    pub id: String,   // Column names "id"
    pub name: String, // Column names "name"
    pub status: String, // Column names "status"
}

pub struct Text {
    pub id: i32,            // Column names "id"
    pub project_id: String, // Column names "projectId"
    pub value: String,      // Column names "value"
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

pub async fn get_text_by_project_id(pool: &Pool<Postgres>, project_id: &str) -> Vec<Text> {
    let q_res = sqlx::query(r#"SELECT * FROM "Text" WHERE "projectId" = $1"#)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .unwrap();

    let mut texts: Vec<Text> = Vec::new();

    for row in q_res {
        let id = row.get("id");
        let value: String = row.get("value");
        let project_id: String = row.get("projectId");
        texts.push(Text {
            id: id,
            project_id: project_id,
            value: value,
        });
    }
    texts
}

pub async fn get_project_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Project, SimpleError> {
    let q_res = sqlx::query(r#"SELECT * FROM "Project" WHERE id = $1 LIMIT 1"#)
        .bind(id)
        .fetch_all(pool)
        .await
        .unwrap();

    let mut project = Project {
        id: "".to_string(),
        name: "".to_string(),
        status: "".to_string(),
    };

    if q_res.len() == 0 {
        return Err(SimpleError::new("Project not found"));
    }

    for row in q_res {
        let id: String = row.get("id");
        let name: String = row.get("name");
        let status: String = row.get("status");
        project = Project { id, name, status };
    }
    Ok(project)
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

pub async fn insert_text_embeddings_by_id(
    trx: &mut Transaction<'_, Postgres>,
    id: i32,
    embeddings: Vec<f32>,
) -> Result<u64, sqlx::Error> {
    let embeddings_json = json!(embeddings);

    let res = sqlx::query(
        r#"
        UPDATE 
            "Text" 
        SET 
            embeddings = $1
        WHERE 
            id = $2
        RETURNING id"#,
    )
    .bind(embeddings_json)
    .bind(id)
    .execute(&mut **trx)
    .await;

    match res {
        Ok(v) => Ok(v.rows_affected()),
        Err(e) => Err(e),
    }
}
