use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    routes: Vec<Route>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    name: String,
    from: String,
    to: String,
}
