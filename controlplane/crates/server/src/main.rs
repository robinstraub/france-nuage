use std::sync::Arc;

use server::auth::openid::OpenID;
use server::handlers::resource_manager::ResourceManagerHandler;
use server::proto::resource_manager_service_server::ResourceManagerServiceServer;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let oidc_issuer_url =
        std::env::var("OIDC_ISSUER_URL").expect("OIDC_ISSUER_URL must be set");
    let oidc_discovery_url = format!("{oidc_issuer_url}/.well-known/openid-configuration");
    let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:50052".into());

    let pool = sqlx::PgPool::connect(&database_url).await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;

    let openid = OpenID::discover(reqwest::Client::new(), &oidc_discovery_url).await?;

    let user_repository: Arc<dyn domain::ports::UserRepository> =
        Arc::new(infra_db::PgUserRepository::new(pool.clone()));
    let organization_repository: Arc<dyn domain::ports::OrganizationRepository> =
        Arc::new(infra_db::PgOrganizationRepository::new(pool.clone()));

    let resource_manager =
        ResourceManagerHandler::new(openid, user_repository, organization_repository);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any)
        .expose_headers(Any);

    let addr = listen_addr.parse()?;
    tracing::info!("france-nuage control plane listening on {addr}");

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(ResourceManagerServiceServer::new(resource_manager))
        .serve(addr)
        .await?;

    Ok(())
}
