use crate::server_fns::{add_todo, Todo};
use leptos::prelude::*;

#[component]
pub fn Header(
    #[prop(into)] on_add: Callback<Todo>,
) -> impl IntoView {
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let (error, set_error) = signal(Option::<String>::None);

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            if let Some(input) = input_ref.get() {
                let val = input.value();
                let trimmed = val.trim().to_string();
                if !trimmed.is_empty() {
                    let on_add = on_add.clone();
                    let input_ref = input_ref.clone();
                    leptos::task::spawn_local(async move {
                        match add_todo(trimmed).await {
                            Ok(todo) => {
                                on_add.run(todo);
                                if let Some(input) = input_ref.get() {
                                    input.set_value("");
                                }
                            }
                            Err(e) => {
                                set_error.set(Some(e.to_string()));
                            }
                        }
                    });
                }
            }
        }
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autofocus=true
                node_ref=input_ref
                on:keydown=on_keydown
            />
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
        </header>
    }
}
