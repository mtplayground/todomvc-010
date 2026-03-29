use crate::server_fns::{delete_todo, toggle_todo, update_todo, Todo};
use leptos::prelude::*;

#[component]
pub fn TodoItem(
    todo: Todo,
    #[prop(into)] on_toggle: Callback<i64>,
    #[prop(into)] on_delete: Callback<i64>,
    #[prop(into)] on_update: Callback<(i64, String)>,
) -> impl IntoView {
    let todo_id = todo.id;
    let (completed, set_completed) = signal(todo.completed);
    let (editing, set_editing) = signal(false);
    let (title, set_title) = signal(todo.title.clone());
    let edit_ref = NodeRef::<leptos::html::Input>::new();

    let on_toggle_click = move |_| {
        let new_completed = !completed.get();
        set_completed.set(new_completed);
        let on_toggle = on_toggle.clone();
        leptos::task::spawn_local(async move {
            if let Err(e) = toggle_todo(todo_id).await {
                tracing::error!("Failed to toggle todo: {}", e);
                // Revert optimistic update
                set_completed.set(!new_completed);
            } else {
                on_toggle.run(todo_id);
            }
        });
    };

    let on_delete_click = move |_| {
        let on_delete = on_delete.clone();
        leptos::task::spawn_local(async move {
            if let Err(e) = delete_todo(todo_id).await {
                tracing::error!("Failed to delete todo: {}", e);
            } else {
                on_delete.run(todo_id);
            }
        });
    };

    let start_edit = move |_| {
        set_editing.set(true);
        // Focus the edit input after render
        leptos::task::spawn_local(async move {
            if let Some(input) = edit_ref.get() {
                let _ = input.focus();
            }
        });
    };

    let save_edit = move |new_title: String| {
        let trimmed = new_title.trim().to_string();
        if trimmed.is_empty() {
            // Empty title - delete the todo
            let on_delete = on_delete.clone();
            leptos::task::spawn_local(async move {
                if let Err(e) = delete_todo(todo_id).await {
                    tracing::error!("Failed to delete todo: {}", e);
                } else {
                    on_delete.run(todo_id);
                }
            });
        } else {
            set_title.set(trimmed.clone());
            set_editing.set(false);
            let on_update = on_update.clone();
            leptos::task::spawn_local(async move {
                if let Err(e) = update_todo(todo_id, trimmed.clone()).await {
                    tracing::error!("Failed to update todo: {}", e);
                } else {
                    on_update.run((todo_id, trimmed));
                }
            });
        }
    };

    let on_edit_keydown = {
        let save_edit = save_edit.clone();
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Enter" {
                if let Some(input) = edit_ref.get() {
                    let val = input.value();
                    save_edit(val);
                }
            } else if ev.key() == "Escape" {
                set_editing.set(false);
            }
        }
    };

    let on_edit_blur = {
        let save_edit = save_edit.clone();
        move |_: leptos::ev::FocusEvent| {
            if editing.get() {
                if let Some(input) = edit_ref.get() {
                    let val = input.value();
                    save_edit(val);
                }
            }
        }
    };

    view! {
        <li class=move || {
            let mut classes = vec![];
            if completed.get() { classes.push("completed"); }
            if editing.get() { classes.push("editing"); }
            classes.join(" ")
        }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=move || completed.get()
                    on:change=on_toggle_click
                />
                <label on:dblclick=start_edit>
                    {move || title.get()}
                </label>
                <button class="destroy" on:click=on_delete_click/>
            </div>
            {move || {
                if editing.get() {
                    view! {
                        <input
                            class="edit"
                            node_ref=edit_ref
                            prop:value=move || title.get()
                            on:keydown=on_edit_keydown.clone()
                            on:blur=on_edit_blur.clone()
                        />
                    }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }
            }}
        </li>
    }
}
