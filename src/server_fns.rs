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

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    use leptos::context::use_context;
    use sqlx::{Row, SqlitePool};

    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::new("Title cannot be empty"));
    }

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    // Get the max position so we can add to the end
    let max_pos_row = sqlx::query("SELECT COALESCE(MAX(position), -1) as max_pos FROM todos")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let max_pos: i64 = max_pos_row.try_get("max_pos").unwrap_or(-1);
    let next_pos = max_pos + 1;

    let result = sqlx::query(
        "INSERT INTO todos (title, completed, position) VALUES (?, 0, ?) RETURNING id, title, completed, position"
    )
    .bind(&title)
    .bind(next_pos)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let completed: i64 = result.try_get("completed").unwrap_or(0);
    Ok(Todo {
        id: result.try_get("id").unwrap_or(0),
        title: result.try_get("title").unwrap_or_default(),
        completed: completed != 0,
        position: result.try_get("position").unwrap_or(0),
    })
}

#[server(UpdateTodo, "/api")]
pub async fn update_todo(id: i64, title: String) -> Result<(), ServerFnError> {
    use leptos::context::use_context;
    use sqlx::SqlitePool;

    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::new("Title cannot be empty"));
    }

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
        .bind(&title)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    use leptos::context::use_context;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(ToggleTodo, "/api")]
pub async fn toggle_todo(id: i64) -> Result<(), ServerFnError> {
    use leptos::context::use_context;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    sqlx::query("UPDATE todos SET completed = NOT completed WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(ToggleAll, "/api")]
pub async fn toggle_all(completed: bool) -> Result<(), ServerFnError> {
    use leptos::context::use_context;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    let completed_val: i64 = if completed { 1 } else { 0 };
    sqlx::query("UPDATE todos SET completed = ?")
        .bind(completed_val)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(ClearCompleted, "/api")]
pub async fn clear_completed() -> Result<(), ServerFnError> {
    use leptos::context::use_context;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    sqlx::query("DELETE FROM todos WHERE completed = 1")
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
