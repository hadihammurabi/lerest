extern crate gtk;
use gtk::prelude::*;
use ureq;

fn get_req_method_from_index(index: u8) -> String {
  let method = match index {
    1 => "POST",
    2 => "PUT",
    3 => "PATCH",
    4 => "DELETE",
    _u8 => "GET",
  };

  method.to_string()
}

pub fn build(app: &gtk::Application) {
  let glade_src = include_str!("main_window.ui");
  let builder = gtk::Builder::from_string(glade_src);
  let win: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
  win.set_application(Some(app));

  let btn_req_send: gtk::Button = builder.get_object("btn_req_send").unwrap();
  let text_res_data: gtk::TextView = builder.get_object("text_res_data").unwrap();
  let entry_req_uri: gtk::Entry = builder.get_object("entry_req_uri").unwrap();
  let combo_req_method: gtk::ComboBox = builder.get_object("combo_req_method").unwrap();
  let label_res_status: gtk::Label = builder.get_object("label_res_status").unwrap();
  
  btn_req_send.connect_clicked(move |_btn| {
    let req_method: &str = &get_req_method_from_index(combo_req_method.get_active().unwrap() as u8);
    let data = ureq::builder()
      .build()
      .request(req_method, &*entry_req_uri.get_text().to_string())
      .call()
      .unwrap_or(ureq::Response::new(400, "bad request", "").unwrap());

      let status_text = format!("Status: {}", data.status());
      label_res_status.set_text(&status_text);

      let body = data.into_string().unwrap();
      text_res_data.get_buffer().unwrap().set_text(&body);
  });

  win.show_all();
}
