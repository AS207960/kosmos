use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, default_value = "[::]:10800")]
    listen_address: std::net::SocketAddr,

    #[arg(long, short)]
    db_url: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    if !tokio::task::block_in_place(|| {
        kosmos::run_migrations(&args.db_url)
    }) {
        return
    }

    let db_config = diesel_async::pooled_connection::AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(args.db_url);
    let db_pool = std::sync::Arc::new(
        diesel_async::pooled_connection::mobc::Pool::new(db_config)
    );

    kosmos::mo::receive_mt(args.listen_address, db_pool).await;
}