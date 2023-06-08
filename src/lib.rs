#![no_std]
use gstd::{prelude::*,msg};
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
            msg::reply(String::from("CreatePost"),0).expect("Error while sending reply");
        },
        PostAction::DonateToPoster(ammount) =>{
            msg::reply(String::from("DonateToPoster"),0).expect("Error while sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn metahash(){
    let metahash : [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share metahash");
}