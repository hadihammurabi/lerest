pub fn get_req_method_from_index(index: u8) -> String {
  let method = match index {
    1 => "POST",
    2 => "PUT",
    3 => "PATCH",
    4 => "DELETE",
    _u8 => "GET",
  };

  method.to_string()
}

pub fn http(req_method: &str, uri: &str) -> Result<ureq::Response, ureq::Error> {
  ureq::builder()
    .build()
    .request(req_method, uri)
    .call()
}
