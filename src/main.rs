mod api;
mod database;
mod model;
mod util;

use api::route::login::LoginRoute;
use poem::{
    listener::TcpListener, middleware::Cors, EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    log::info!("Establishing database connection...");
    // let pool = PgPool::connect(&db_url).await?;
    log::info!("Connected to the database");

    // initialize your app's context here
    // wrap it in Arc in case of problems
    // use poem::Data<Context> in order to extract it

    // define your `routes` tuple here

    let addr =
        std::env::var("LISTEN_ADDRESS").unwrap_or("0.0.0.0:8000".to_string());
    let mut api_endpoint = addr.clone();
    api_endpoint.push_str("/api");

    // replace empty tuple with the `routes` tuple
    let api_service = OpenApiService::new(LoginRoute, "ITS", "1.0.0")
        .server("http://localhost:8000/api");
    let ui = api_service.swagger_ui();
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/docs", ui)
                .with(
                    Cors::new()
                ),
        )
        .await?;
    Ok(())
}
