#[warn(unused_imports)]

use serde::{Deserialize, Serialize};
use cdrs::frame::{IntoBytes, TryFromRow};
use cdrs::query_values;
use cdrs::query::QueryValues;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::ByName;

use uuid::Uuid;
use blob_uuid::{to_blob};
use chrono::{DateTime, NaiveDateTime};
use chrono::prelude::*;

use std::env;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Twin {
  id: Uuid,
  name: String,
  created_at: DateTime<Utc>,
  owner: Uuid
}

/// Generic element component of a Twin instance.
/// Used to define structure between other elements and to attach sources of data.
#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, Eq, Ord, PartialEq, PartialOrd)]
pub struct Element {
  pub id: Uuid,
  pub twin: Uuid,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent: Option<Uuid>, // optional parent element
  // pub created_at: time::OffsetDateTime
  // pub created_at: DateTime<Utc>
}

impl Element {
  // fn new(id: Uuid, twin: Uuid, name: String, parent: Uuid, created_at: Timespec) -> Element {
  // fn new(id: Uuid, twin: Uuid, name: String, parent: Uuid, created_at: DateTime<Utc>) -> Element {
  fn new(id: Uuid, twin: Uuid, name: String, parent: Uuid) -> Element {
    Element {
      id: id,
      twin: twin,
      name: name,
      parent: Some(parent),
      // created_at: created_at
    }
  }

  pub fn to_query(self) -> QueryValues {
    query_values!(
      "id" => self.id,
      "twin" => self.twin,
      "name" => self.name,
      "parent" => self.parent
      // "created_at" => self.created_at,
    )
  }

  pub fn to_query_no_parent(self) -> QueryValues {
    query_values!(
      "id" => self.id,
      "twin" => self.twin,
      "name" => self.name
      // "created_at" => self.created_at,
    )
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementSerialized {
  pub element: Element,
  pub sources: Vec<Source>,
  pub children: Vec<ElementSerialized>
}

impl ElementSerialized {
  pub fn add_child(&mut self, child_element: ElementSerialized) {
    self.children.push(child_element);
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementRegister {
  pub name: String,
  pub parent: Option<Uuid>
}

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Source {
  pub id: Uuid,
  pub name: String,
  pub element: Uuid,
  // pub created_at: DateTime<Utc>
  // type
}

impl Source {
  pub fn to_query(self) -> QueryValues {
    query_values!(
      "id" => self.id,
      "name" => self.name,
      "element" => self.element
    )
  }

  pub fn data_topic(self) -> String {
    let twin = Uuid::parse_str(env::var("TWIN_INSTANCE").unwrap().as_str())
      .expect("Twin instance to be defined");

    format!("{}/{}/{}", to_blob(&twin), to_blob(&self.element), to_blob(&self.id))
  }

  pub fn data_ack_topic(self) -> String {
    format!("{}/ack", self.data_topic())
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceRegister {
  pub name: String,
  pub element: Uuid
  // type
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceData {
  pub source: Uuid,
  pub stamp: i64, // DateTime<Utc>
  pub value: String,
  pub created_at: i64 // DateTime<Utc>
}

// impl TryFromRow for SourceData {
//   fn try_from_row(row: cdrs::types::rows::Row) -> cdrs::Result<Self> {
//     Ok(SourceData {
//       source: row.by_name("source").unwrap().expect("Source id"),
//       stamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(row.by_name("stamp").unwrap().expect("Timestamp"), 0), Utc),
//       value: row.by_name("value").unwrap().expect("Value"),
//       created_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(row.by_name("created_at").unwrap().expect("Created at"), 0), Utc)
//     })
//   }
// }

impl SourceData {
  pub fn get_stamp(&self) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.stamp / 1000, 0), Utc)
  }

  pub fn get_created_at(&self) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.created_at / 1000, 0), Utc)
  }
}
