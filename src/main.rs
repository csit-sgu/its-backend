mod api;
mod database;
mod model;
mod util;

use std::sync::Arc;

use crate::{
    api::route::{index::IndexRoute, tasks::TasksRoute},
    database::AggregationRepo,
    util::Context,
};

use api::route::login::LoginRoute;
use poem::{
    listener::TcpListener, middleware::Cors, EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use sqlx::{MySqlPool, PgPool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    dotenv::dotenv()?;

    let mysql_url = std::env::var("MYSQL_URL")?;
    let pg_url = std::env::var("POSTGRES_URL")?;
    log::info!("Establishing MySQL database connection...");
    let mysql_pool = MySqlPool::connect(&mysql_url).await?;
    log::info!("Connected to MySQL");
    log::info!("Establishing Postgres database connection...");
    let pg_pool = PgPool::connect(&pg_url).await?;
    log::info!("Connected to Postgres");

    let aggregation_repo = AggregationRepo {
        mysql_pool,
        pg_pool,
    };

    let ctx = Arc::new(Context { aggregation_repo });
    let routes = (IndexRoute, TasksRoute { ctx }, LoginRoute);

    let listen_addr = std::env::var("LISTEN_ADDRESS")?;
    log::info!("{}", listen_addr);
    let api_addr = std::env::var("API_URL")?;
    log::info!("{}", api_addr);

    let api_service =
        OpenApiService::new(routes, "ITS", "1.0.0").server(api_addr);
    let ui = api_service.swagger_ui();
    Server::new(TcpListener::bind(listen_addr))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/docs", ui)
                .with(Cors::new()),
        )
        .await?;

    Ok(())
}
