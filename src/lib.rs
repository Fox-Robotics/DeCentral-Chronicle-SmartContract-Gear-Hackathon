#![no_std]
use gstd::{prelude::*,msg, exec};
use chronicles_io::{PostAction,IoPost};



static mut POSTS : Vec<IoPost> = Vec::new();


#[no_mangle]
extern "C" fn init(){}

#[no_mangle]
extern "C" fn handle(){
    /*msg::reply(String::from("Hello People"),0).expect("Errro while trying to send reply");
    let payload_string: String = msg::load().expect("Unable to decode `String`");
    msg::reply(payload_string,0).expect("Errro while trying to send reply");*/

    let message : PostAction = msg::load().expect("Unable to decode PostAction object");
    match message{
        PostAction::CreatePost(title,body) =>{
            
            let newPost = IoPost{
                title: title,
                body: body,
                poster_wallet: msg::source()
            };
            
            unsafe{POSTS.push(newPost)};
            msg::reply(unsafe{POSTS.clone()},0).expect("Failed to share state");
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
    msg::reply(unsafe{POSTS.clone()},0).expect("Failed to share state");
}