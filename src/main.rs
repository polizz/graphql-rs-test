use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql::{ComplexObject, Number, Parents, SimpleObject};
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use sqlx::SqlitePool;
use std::error::Error;

#[derive(sqlx::FromRow, Debug, SimpleObject)]
#[graphql(complex)]
struct Customer {
    CustomerId: i64,
    FirstName: String,
    LastName: String,
    PostalCode: String,
    Fax: String,
    Email: String,
}

#[ComplexObject]
impl Customer {
    async fn customer_invoices<'c>(
        &self,
        ctx: &Context<'c>,
        customer_id: i64,
    ) -> Result<Vec<Invoice>, anyhow::Error> {
        let conn = ctx.data_unchecked::<SqlitePool>();

        sqlx::query_as::<_, Invoice>("Select * from Invoice where CustomerId = $1")
            // .bind(&self.CustomerId)
            .bind(customer_id)
            .fetch_all(conn)
            .await
            .map_err(|e| e.into())
    }
}

#[derive(sqlx::FromRow, Debug, SimpleObject)]
struct Invoice {
    InvoiceDate: String,
    Total: f64,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn customers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        customer_id: Option<i64>,
    ) -> Result<Vec<Customer>, anyhow::Error> {
        // async fn load_all(&self) -> Vec<Self> {
        let conn = ctx.data_unchecked::<SqlitePool>();
        // sqlx::query_as::<_, Customer>("Select LastName, FirstName from Customer limit 10;")
        // let parent_id = ctx.path_node;
        // dbg!(parent_id);
        sqlx::query_as::<_, Customer>("Select * from Customer;")
            .fetch_all(conn)
            .await
            .map_err(|e| e.into())

        // if (ctx.look_ahead().field("invoices")) {
        //     sqlx::query_as::<_, Customer>("Select * from Customer;")
        //         .fetch_all(conn)
        //         .await
        //         .map_err(|e| e.into())
        // } else {
        //     sqlx::query_as::<_, Customer>("Select * from Customer;")
        //         .fetch_all(conn)
        //         .await
        //         .map_err(|e| e.into())
        // }
    }

    async fn invoices<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Invoice>, anyhow::Error> {
        let conn = ctx.data_unchecked::<SqlitePool>();
        let parent_id = ctx.path_node;
        dbg!(parent_id);

        sqlx::query_as::<_, Invoice>("Select * from Invoice;")
            .fetch_all(conn)
            .await
            .map_err(|e| e.into())
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = SqlitePool::connect("Chinook_Sqlite.sqlite").await?;
    // let conn = Sqlite::Connection::connect("sqlite::memory:").await?;

    // create the schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(conn)
        .finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
