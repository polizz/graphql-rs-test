use async_graphql::extensions::Tracing as AsyncTracing;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use std::error::Error;
use tracing_subscriber::EnvFilter;

pub mod model;
use model::QueryRoot;

pub mod data;
use data::{CategoriesContainer, ItemsContainer, ProductsContainer};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(level) = std::env::var("RUST_LOG") {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new(&format!("{level}")))
            .init();
    }

    // create the schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(ProductsContainer::new())
        .data(CategoriesContainer::new())
        .data(ItemsContainer::new())
        .extension(AsyncTracing)
        .finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    let s1 = Server::new(TcpListener::bind("0.0.0.0:8000")).run(app);
    // .await?;

    futures::future::join_all(vec![s1]).await;

    Ok(())
}
