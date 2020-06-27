use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::QueryExecutor;
use cdrs::frame::traits::TryFromRow;

use std::sync::Arc;
use std::env;

use uuid::Uuid;
use blob_uuid::to_uuid;

use actix_web::{web};

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

pub fn get_db_session() -> Arc<CurrentSession> {
  let _session = init_db_session();

  assert!(_session.query("USE dt;").is_ok(), "Should have set keyspace.");

  _session
}

pub fn get_item_by_id<T: TryFromRow>(session: web::Data<Arc<CurrentSession>>, item_id: String, table: String) -> Result<T, (String, usize)> {
  let id: Uuid;
  
  match Uuid::parse_str(&item_id) {
    Ok(_id) => { id = _id },
    Err(_error) => {
      match to_uuid(&item_id) {
        Ok(_id) => { id = _id },
        Err(_) => { return Err((format!("Invalid input."), 400)); }
      }
    }
  }

  let r = session.query(format!("SELECT * FROM {} WHERE id = {}", table, id));

  let rows = r.expect("Get item by id")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Err(("No item found.".to_string(), 404));
  }
  return Ok(T::try_from_row(rows[0].clone()).unwrap());
}
