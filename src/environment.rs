use std::env;

#[derive(Debug, Clone)]
pub struct Env {
    pub name: String,
    pub host: String,
    pub port: String,
    pub db: String,
}

pub fn parse_env() -> Env {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or("authrs=debug,tower_http=debug".to_string()),
    );

    let name = match env::var("NAME") {
        Ok(v) => v,
        Err(_) => "authrs".to_string(),
    };

    let host = match env::var("HOST") {
        Ok(v) => v,
        Err(_) => "127.0.0.1".to_string(),
    };

    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(_) => "3000".to_string(),
    };

    let db = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => "./auth.db3".to_string(),
    };

    Env {
        name,
        host,
        port,
        db,
    }
}

fn parse_env_var_u32(var: &str, default: u32) -> u32 {
    let result = match env::var(var) {
        Ok(v) => match v.parse::<u32>() {
            Ok(v) => v,
            Err(err) => {
                tracing::error!("Parse {} error: {}", var, err);
                default
            }
        },
        Err(_) => default,
    };
    result
}

fn parse_env_var_bool(var: &str, default: u8) -> bool {
    let res = if parse_env_var_u8(var, default) == 1 {
        true
    } else {
        false
    };
    res
}

fn parse_env_var_u8(var: &str, default: u8) -> u8 {
    let result = match env::var(var) {
        Ok(v) => match v.parse::<u8>() {
            Ok(v) => v,
            Err(err) => {
                tracing::error!("Parse {} error: {}", var, err);
                default
            }
        },
        Err(_) => default,
    };
    result
}
