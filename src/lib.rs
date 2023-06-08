#![no_std]
use gstd::{prelude::*,msg, exec};
use chronicles_io::PostAction;

#[no_mangle]
extern "C" fn init(){}

#[no_mangle]
extern "C" fn handle(){
    /*msg::reply(String::from("Hello People"),0).expect("Errro while trying to send reply");
    let payload_string: String = msg::load().expect("Unable to decode `String`");
    msg::reply(payload_string,0).expect("Errro while trying to send reply");*/

    let message : PostAction = msg::load().expect("Unable to decode PostAction object");
    match message{
        PostAction::CreatePost(content) =>{
            //msg::reply(format!("{}", msg::source()),0).expect("Error while sending reply");
            msg::reply(format!("Echo: {}", content),0).expect("Error while sending reply");
        },
        PostAction::DonateToPoster(ammount) =>{
            msg::reply(String::from("DonateToPoster"),0).expect("Error while sending reply");
            exec::exit(msg::source());
        }
    }
}

#[no_mangle]
extern "C" fn metahash(){
    let metahash : [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share metahash");
}


#[no_mangle]
extern "C" fn state() {
    msg::reply("{\"field1\": 123 }",0).expect("Failed to share state");
    //msg::reply(unsafe { WALLETS.clone() }, 0).expect("Failed to share state");
}