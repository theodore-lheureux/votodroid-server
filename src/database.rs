use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .build(mgr)
        .expect("could not build connection pool");
    pool
}
