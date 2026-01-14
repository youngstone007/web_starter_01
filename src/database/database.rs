use crate::config;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use serde::de::Unexpected::Str;
use std::cmp::max;
use std::time::Duration;

pub async fn init_database() -> anyhow::Result<DatabaseConnection> {
    let database_config = config::config().database();
    // mysql://用户名:密码@主机名:端口/数据库名
    // mysql://username:password@host/database
    let mut options = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        database_config.user(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database()
    ));

    let cpus = num_cpus::get() as u32;

    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 5, 20))
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(options).await?;

    tracing::info!("database connection established");

    // print the version of db
    print_dbversion(&db).await?;

    Ok(db)
}

async fn print_dbversion(db: &DatabaseConnection) -> anyhow::Result<()> {
    let query_result = db
        .query_one(Statement::from_string(DbBackend::MySql, "Select version()"))
        .await?
        .ok_or_else(|| anyhow::anyhow!("No version found"))?;

    let version = query_result.try_get_by_index::<String>(0)?;
    tracing::info!("the mysql version: {}", version);
    Ok(())
}
