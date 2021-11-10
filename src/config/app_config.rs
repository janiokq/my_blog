use std::fs::File;
use std::io::prelude::*;
use toml;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConf {
    pub name: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub jwt: JWTConf,
    pub mongo: MongoConf,
    pub server: ServerConf,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWTConf {
    pub site_key: String,
    pub claim_exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoConf {
    pub ip: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConf {
    pub port: Option<u16>,
    pub page_url: Option<String>,
    pub server_url: Option<String>,
    pub web_path: Option<String>, // 静态资源路径
}

impl AppConf {
    pub fn new(file_path:&str) -> Self{
        let mut file = match File::open(file_path) {
            Ok(f)=> f,
            Err(e)=> panic!("no such file {} exception:{} ",file_path,e),
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        let app: AppConf = toml::from_str(&str_val).unwrap();

        app
    }
}