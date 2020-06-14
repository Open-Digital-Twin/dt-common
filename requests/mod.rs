#[cfg(test)] // only used in testing. Remove otherwise.
use reqwest::blocking::{Client, RequestBuilder};

#[cfg(test)] // only used in testing. Remove otherwise.
use std::env;

#[cfg(test)]
#[allow(dead_code)]
fn get_api() -> String {
  let api = env::var("SERVER_ADDRESS").expect("Test API address.");

  api
}

#[cfg(test)]
#[allow(dead_code)]
fn create_client() -> Client {
  Client::new()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn get(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  create_client().get(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn post(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  create_client().post(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn put(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  create_client().put(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn delete(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  create_client().delete(&url)
}
