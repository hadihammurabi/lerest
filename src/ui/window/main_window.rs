extern crate gtk;
use gtk::prelude::*;
use reqwest;

pub fn build(app: &gtk::Application) {
  let glade_src = include_str!("main_window.ui");
  let builder = gtk::Builder::from_string(glade_src);
  let win: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
  win.set_application(Some(app));

  let btn_req_send: gtk::Button = builder.get_object("btn_req_send").unwrap();
  btn_req_send.connect_clicked(move |_btn| {
    let text_res_data: gtk::TextView = builder.get_object("text_res_data").unwrap();
    let entry_req_uri: gtk::Entry = builder.get_object("entry_req_uri").unwrap();

    let data = reqwest::blocking::get(&*entry_req_uri.get_text().to_string()).unwrap();
    text_res_data.get_buffer().unwrap().set_text(&data.text().unwrap());
  });

  win.show_all();
}
