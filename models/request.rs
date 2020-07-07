#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Serializeable {
  pub serialized: bool
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DataInterval {
  pub since: Option<i64>,
  pub until: Option<i64>,
  pub force: Option<bool>
}
