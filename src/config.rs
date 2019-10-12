use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub basic: BasicCfg,
}


#[derive(Debug, Deserialize)]
pub struct BasicCfg {
    pub database_url: String,
}


