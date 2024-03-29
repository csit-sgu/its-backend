mod api;
mod database;
mod model;
mod processing;
mod util;

use std::sync::Arc;

use crate::{
    api::route::{
        index::IndexRoute, tasks::TasksRoute, transitions::TransitionRoute,
    },
    database::{AggregationRepo, TransitionRepo},
    processing::extractor::MetricExtractor,
    util::Context,
};

use api::route::login::LoginRoute;
use poem::{
    listener::TcpListener, middleware::Cors, EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    dotenv::dotenv()?;

    let mysql_url = std::env::var("MYSQL_URL")?;
    log::info!("Establishing MySQL database connection...");
    let mysql_pool = MySqlPoolOptions::new()
        .max_lifetime(Duration::from_secs(12000))
        .connect(&mysql_url)
        .await?;
    log::info!("Connected to MySQL");

    let aggregation_repo = AggregationRepo {
        mysql_pool: mysql_pool.clone(),
    };

    let transition_repo = TransitionRepo { mysql_pool };
    let metric_extractor = MetricExtractor;

    let ctx = Arc::new(Context {
        aggregation_repo,
        transition_repo,
        metric_extractor,
    });
    let routes = (
        IndexRoute,
        TasksRoute { ctx: ctx.clone() },
        LoginRoute,
        TransitionRoute { ctx },
    );

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
