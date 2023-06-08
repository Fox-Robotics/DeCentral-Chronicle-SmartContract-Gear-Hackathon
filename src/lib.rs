#![no_std]
use gstd::{prelude::*,msg};

#[no_mangle]
extern "C" fn init(){}

#[no_mangle]
extern "C" fn handle(){
    msg::reply(String::from("Hello People"),0).expect("Errro while trying to send reply");
    let payload_string: String = msg::load().expect("Unable to decode `String`");
    msg::reply(payload_string,0).expect("Errro while trying to send reply");
}