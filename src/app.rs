use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="todomvc-common" href="https://unpkg.com/todomvc-common@1.0.5/base.css"/>
        <Stylesheet id="todomvc-app-css" href="https://unpkg.com/todomvc-app-css@2.4.2/index.css"/>
        <Title text="TodoMVC - Leptos"/>
        <Router>
            <Routes fallback=|| view! { <p class="error">"Page not found"</p> }>
                <Route path=path!("/") view=TodoApp/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
            </header>
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with "<a href="https://leptos.dev">"Leptos"</a></p>
            <p>"Part of "<a href="http://todomvc.com">"TodoMVC"</a></p>
        </footer>
    }
}
