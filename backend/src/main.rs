use std::sync::Arc;

use axum::Router;
use clap::{Args, Parser, Subcommand};
use tokio::sync::Mutex;

use crate::{
    currency::{Currency, ParseCurrencyError},
    db::Db,
};

mod api;
mod auth;
mod category;
mod currency;
mod db;
mod money;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 2667; // bons on a phone dial

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve(ServeArgs),
    Create {
        #[command(subcommand)]
        command: CreateCommand,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Serve(args) => serve(args).await?,
        Command::Create {
            command: CreateCommand::User(args),
        } => create_user(args).await?,
    }

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Db>>,
    currency: Currency,
}

#[derive(Args)]
struct ServeArgs {
    #[arg(long, default_value = DEFAULT_HOST)]
    host: String,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

async fn serve(args: ServeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let currency = std::env::var("BONS_CURRENCY")
        .map_err(|_| ParseCurrencyError)?
        .parse::<Currency>()?;
    if let Currency::Unknown(code) = &currency {
        eprintln!("Warning: BONS_CURRENCY contains an unknown currency code: {code:?}");
    }

    let db = Db::connect().await?;
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        currency,
    };

    let app = Router::new()
        .nest("/api", api::router(state.clone()))
        .with_state(state);

    let address = format!("{}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("serving on http://{address}");

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Subcommand)]
enum CreateCommand {
    User(CreateUserArgs),
}

#[derive(Args)]
struct CreateUserArgs {
    name: String,

    #[arg(long)]
    admin: bool,
}

async fn create_user(args: CreateUserArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::connect().await?;
    let created_user = db.create_user(args.name, args.admin).await?;

    println!("created user: {}", created_user.user.name);
    println!("admin: {}", created_user.user.is_admin);
    println!("key: {}", created_user.key_as_str());

    Ok(())
}
