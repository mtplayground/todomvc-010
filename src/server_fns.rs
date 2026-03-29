use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: i64,
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use leptos::context::use_context;
    use sqlx::{Row, SqlitePool};

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    let rows = sqlx::query(
        "SELECT id, title, completed, position FROM todos ORDER BY position ASC, id ASC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let todos = rows
        .into_iter()
        .map(|row| {
            let completed: i64 = row.try_get("completed").unwrap_or(0);
            Todo {
                id: row.try_get("id").unwrap_or(0),
                title: row.try_get("title").unwrap_or_default(),
                completed: completed != 0,
                position: row.try_get("position").unwrap_or(0),
            }
        })
        .collect();

    Ok(todos)
}
