use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/todomvc-010.css"/>
        <Title text="TodoMVC - Leptos"/>
        <Router>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <Route path=path!("/") view=|| view! { <p>"Loading..."</p> }/>
            </Routes>
        </Router>
    }
}
