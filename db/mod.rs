use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::QueryExecutor;

use std::sync::Arc;
use std::env;

// Requires mod models
use crate::common::models::app::{CurrentSession};

pub fn init_db_session() -> Arc<CurrentSession> {
  let db_address = env::var("DB_ADDRESS").unwrap();

  let node = NodeTcpConfigBuilder::new(
    &db_address,
    NoneAuthenticator {}
  ).build();

  let cluster_config = ClusterTcpConfig(vec![node]);

  let _session: Arc<CurrentSession> = Arc::new(
    new_session(&cluster_config, RoundRobin::new())
      .expect("session should be created")
  );

  _session
}

#[allow(dead_code)]
pub fn get_db_session() -> Arc<CurrentSession> {
  let _session = init_db_session();

  assert!(_session.query("USE dt;").is_ok(), "Should have set keyspace.");

  _session
}
