use crate::db::connect::DBConfig;
use anyhow::Context;
use std::env;
use std::error::Error as StdError;
use std::fmt;

pub enum Platform {
    Local,
    Lambda,
}

impl Platform {
    pub fn get_database_config(&self) -> anyhow::Result<DBConfig> {
        {
            let get_config_string = match self {
                Self::Local => |name| {
                    env::var(name)
                        .with_context(|| format!("Required env var \"{}\" was not set", name))
                },
                _ => unimplemented!(),
            };

            let user = get_config_string("POSTGRES_USER")?;
            let secret = get_config_string("POSTGRES_SECRET")?;
            let host = get_config_string("POSTGRES_HOST")?;
            let port = get_config_string("POSTGRES_PORT")?;
            let port = str::parse(&port)
                .with_context(|| format!("Failed to parse port \"{}\" as a number", port))?;
            let database = get_config_string("POSTGRES_DATABASE")?;
            Ok(DBConfig {
                user,
                secret,
                host,
                port,
                database,
            })
        }
    }
}
