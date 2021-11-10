mod config;
mod common;
mod middleware;
mod model;
mod template;
mod controller;
mod router;

use std::net::SocketAddr;
use config::AppConf;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, options::ServerAddress, Client as MongoClient};
use crate::router::{route_info};

lazy_static! {
    pub static ref GLOBAL_CONF: AppConf = AppConf::new("/Users/weeget/dev/rust/my_blog/config/app.toml");
    // pub static ref GLOBAL_CONF: AppConf = AppConf::new("./app.toml");
    pub static ref MONGO: MongoClient =  {
        let mut options = ClientOptions::default();
        options.hosts = vec![
                      ServerAddress::Tcp {
                          host: GLOBAL_CONF.mongo.ip.to_string(),
                          port: Some(GLOBAL_CONF.mongo.port),
                      }
                  ];
        MongoClient::with_options(options).expect("Failed to initialize standalone client.")
    };
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "static_file_server=debug,tower_http=debug")
    }
    let port = GLOBAL_CONF.server.port.unwrap_or(80u16);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));


    axum_server::bind_rustls(addr.clone().to_string())
        .bind("127.0.0.1:8081").serve()
        .private_key_file("/Users/weeget/dev/rust/my_blog/nginx-selfsigned.key")
        .certificate_file("/Users/weeget/dev/rust/my_blog/nginx-selfsigned.crt")
    // axum::Server::bind(&addr)
        .serve(route_info())
        .await
        .unwrap();
}
