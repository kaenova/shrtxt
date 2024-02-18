use sqlx::{Pool, Postgres, Row, Transaction};

// Table Name "Project"
pub struct Project {
    pub id: String,   // Column names "id"
    pub name: String, // Column names "name"
}

// Table Name "Text"
pub struct Text {
    pub id: i32,              // Column names "id"
    pub project_id: String,   // Column names "projectId"
    pub value: String,        // Column names "value"
    pub embeddings: Vec<f32>, // Column names "embeddings"
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
        Ok(v) => {
            for rec in v {
                let one: i32 = rec.get("one");
                println!("one: {}", one);
            }
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
