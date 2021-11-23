mod config;
mod common;
mod middleware;
mod model;
mod template;
mod controller;
mod router;

use std::net::SocketAddr;
use axum::handler::get;
use axum::http::Uri;
use axum::response::Redirect;
use config::AppConf;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, options::ServerAddress, Client as MongoClient};
use crate::router::{route_info};
use axum::{Router};
use mongodb::options::Credential;

lazy_static! {
    // pub static ref GLOBAL_CONF: AppConf = AppConf::new("/Users/weeget/dev/rust/my_blog/config/app.toml");
    pub static ref GLOBAL_CONF: AppConf = AppConf::new("./app.toml");
    pub static ref MONGO: MongoClient =  {
        let mut options = ClientOptions::default();
        options.hosts = vec![
                      ServerAddress::Tcp {
                          host: GLOBAL_CONF.mongo.ip.to_string(),
                          port: Some(GLOBAL_CONF.mongo.port),
                      }
                  ];
        let mut  credential  = Credential::default();
        // credential.username = Some(GLOBAL_CONF.mongo.username.to_string());
        // credential.password = Some(GLOBAL_CONF.mongo.password.to_string());
        // options.credential = Some(credential);
        MongoClient::with_options(options).expect("Failed to initialize standalone client.")
    };
}

#[tokio::main]
async fn main() {

    // Credential::default().username
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "static_file_server=debug,tower_http=debug")
    }
    let use_https = GLOBAL_CONF.server.use_https.unwrap_or(0u16);
    if use_https == 0 {
        let http = tokio::spawn(http_server(false));
        let _ = tokio::join!(http);
    } else {
        let private_key_file = GLOBAL_CONF.server.private_key_file.as_ref().unwrap();
        let certificate_file = GLOBAL_CONF.server.certificate_file.as_ref().unwrap();
        let http = tokio::spawn(http_server(true));
        let https = tokio::spawn(https_server(private_key_file.to_string(), certificate_file.to_string()));
        let _ = tokio::join!(http, https);
    }
}


async fn http_server(usehttps: bool) {
    let port = GLOBAL_CONF.server.port.unwrap_or(80u16);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("http listening on {}", addr);
    if usehttps {
        let app = Router::new().route("/", get(http_handler));
        axum_server::bind(addr)
            .serve(app)
            .await
            .unwrap();
    }else {
        axum_server::bind(addr)
            .serve(route_info())
            .await
            .unwrap();
    }
}

async fn http_handler(uri: Uri) -> Redirect {
    let port = GLOBAL_CONF.server.https_port.unwrap_or(80u16);
    let domain_name = GLOBAL_CONF.server.domain_name.as_ref().unwrap();
    let uri = format!("https://{}:{}{}", domain_name, port, uri.path())
        .try_into()
        .unwrap();

    Redirect::found(uri)
}

async fn https_server(private_key_file: String, certificate_file: String) {
    let port = GLOBAL_CONF.server.https_port.unwrap_or(443u16);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("https listening on {}", addr);
    axum_server::bind_rustls(addr.clone().to_string())
        .private_key_file(private_key_file)
        .certificate_file(certificate_file)
        .serve(route_info())
        .await
        .unwrap();
}


