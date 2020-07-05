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
use rumq_client::{QoS};

// Requires mod models
use crate::common::models::app::{CurrentSession};

use crate::common::models::twin::{Source, Element};

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

#[allow(dead_code)]
pub fn get_by_id<T: TryFromRow>(session: web::Data<Arc<CurrentSession>>, item_id: String, table: String) -> Result<T, (String, usize)> {
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

#[allow(dead_code)]
pub fn delete_by_id<T: TryFromRow>(session: web::Data<Arc<CurrentSession>>, item_id: String, table: String) -> Result<String, (String, usize)> {
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

  let r = session.query(format!("DELETE FROM {} WHERE id = {}", table, id));

  return match r {
    Ok(_) => Ok(format!("Deleted {} {}.", table, id)),
    Err(_) => Ok(format!("Error deleting {} {}.", table, id))
  };
}

#[allow(dead_code)]
pub fn get_element_sources(session: web::Data<Arc<CurrentSession>>, element_id: String) -> Result<Vec<Source>, (String, usize)> {
  let id: Uuid;

  match Uuid::parse_str(&element_id) {
    Ok(_id) => { id = _id },
    Err(_error) => {
      match to_uuid(&element_id) {
        Ok(_id) => { id = _id },
        Err(_) => { return Err((format!("Invalid input."), 400)); }
      }
    }
  }

  let r = session.query(format!("SELECT * FROM source WHERE element = {}", id));

  let rows = r.expect("Get sources by element")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Ok(vec![]);
  }

  let mut sources: Vec<Source> = Vec::new();
  for row in rows {
    sources.push(Source::try_from_row(row).unwrap());
  }
  Ok(sources)
}

#[allow(dead_code)]
pub fn get_twin_elements(session: web::Data<Arc<CurrentSession>>) -> Result<Vec<Element>, (String, usize)> {
  let twin = env::var("TWIN_INSTANCE").unwrap();

  let r = session.query(format!("SELECT * FROM element WHERE twin = {}", twin));

  let rows = r.expect("Get elements of twin")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Ok(vec![]);
  }

  let mut elements: Vec<Element> = Vec::new();
  for row in rows {
    elements.push(Element::try_from_row(row).unwrap());
  }
  Ok(elements)
}

#[allow(dead_code)]
pub fn get_qos(variable: &str) -> QoS {
  let qos_value = env::var(variable).unwrap().parse::<u8>().unwrap();

  match qos_value {
    0 => QoS::AtMostOnce,
    1 => QoS::AtLeastOnce,
    2 => QoS::ExactlyOnce,
    _ => QoS::AtMostOnce
  }
}
