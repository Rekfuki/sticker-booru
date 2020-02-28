use crate::db::connect::DBConfig;
use anyhow::Context;
use serde_json::Value;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use warp::Filter;

pub enum Platform {
    Local,
    Lambda,
}

impl Platform {
    pub fn autodetect() -> Platform {
        match std::env::var("LAMBDA_TASK_ROOT") {
            Ok(_) => Self::Lambda,
            Err(_) => Self::Local,
        }
    }

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
            let password = get_config_string("POSTGRES_PASSWORD")?;
            let host = get_config_string("POSTGRES_HOST")?;
            let port = get_config_string("POSTGRES_PORT")?;
            let port = str::parse(&port)
                .with_context(|| format!("Failed to parse port \"{}\" as a number", port))?;
            let database = get_config_string("POSTGRES_DATABASE")?;
            Ok(DBConfig {
                user,
                password,
                host,
                port,
                database,
            })
        }
    }

    pub async fn serve<Fut>(self, handler: fn(Value) -> Fut) -> anyhow::Result<()>
    where
        Fut: Future<Output = anyhow::Result<Value>> + Send + 'static,
    {
        match self {
            Self::Lambda => {
                let result = lambda::run(lambda::handler_fn(handler)).await;
                match result {
                    Ok(()) => (),
                    Err(err) => anyhow::bail!(err),
                }

                Ok(())
            }
            Self::Local => {
                let hello = warp::path!("hello")
                .and(warp::body::json())
                .map_async(move |body| async move {
                    let result: Box<dyn warp::reply::Reply> = match handler(body).await {
                        Ok(v) => Box::new(warp::reply::json(&v)),
                        Err(e) => Box::new(warp::reply::with_status(
                            warp::http::Response::new(e.to_string()),
                            warp::http::status::StatusCode::INTERNAL_SERVER_ERROR,
                        )),
                    };
                    result
                });

                warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
                Ok(())
            }
        }
    }
}
