use dotenv::dotenv;
use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};
use serde::Deserialize;
use tokio_postgres::NoTls;

pub type Db = deadpool_postgres::Pool;

#[derive(Debug, Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ::config::ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.set_default("pg.port", "5432").unwrap();
        cfg.merge(::config::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    dotenv().ok();
    let cfg = Config::from_env().unwrap();
    let pool = match cfg.pg.create_pool(NoTls) {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(rocket);
        }
    };

    Ok(rocket.manage(pool))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("Database", init_db))
    })
}
