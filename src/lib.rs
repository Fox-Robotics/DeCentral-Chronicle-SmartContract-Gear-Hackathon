#![no_std]
use gstd::{prelude::*,msg};

#[no_mangle]
extern "C" fn init(){}

#[no_mangle]
extern "C" fn handle(){
    msg::reply(String::from("Hello People"),0).expect("Errro while trying to send reply");
}