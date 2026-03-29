#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use todomvc_010::app::App;
    use todomvc_010::db::create_pool;

    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:todos.db".to_string());
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || {
                    leptos::context::provide_context(pool.clone());
                }
            },
            App,
        )
        .fallback(leptos_axum::file_and_error_handler(|_opts| {
            use leptos::prelude::*;
            use todomvc_010::app::App;
            view! { <App/> }
        }))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
