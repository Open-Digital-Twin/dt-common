use std::env;
use reqwest::blocking::{Client, RequestBuilder}

#[cfg(test)]
#[allow(dead_code)]
fn get_api() -> String {
  let api = env::var("SERVER_ADDRESS").expect("Test API address.");

  api
}

#[cfg(test)]
#[allow(dead_code)]
fn request_create_client() -> Client {
  Client::new()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_get(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().get(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_post(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().post(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_put(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().put(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_delete(addr: &str) -> RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().delete(&url)
}
