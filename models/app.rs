use std::sync::Arc;
use uuid::Uuid;

use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{Session};
use cdrs::cluster::{TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

use serde::{Deserialize};

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

#[allow(dead_code)]
pub struct AppState {
  pub session: Arc<CurrentSession>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Environment {
  pub server_address: String,
  pub db_address: String,
  pub secret_key: String,
  pub twin_instance: Uuid
}

pub const SOURCE_DATA_TOPIC: &str = "Data_Publish";
pub const SOURCE_DATA_ACK_TOPIC: &str = "Data_Publish_ACK";
