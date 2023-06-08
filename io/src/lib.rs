#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId,Encode,Decode,TypeInfo,Debug};

pub struct ChronicleMetadata;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum PostAction{
    CreatePost(String),
    DonateToPoster(u32)
}

impl Metadata for ChronicleMetadata{
    type Init = ();
    type Handle = InOut<PostAction, String>;
    type Reply = ();
    type Others =();
    type Signal = ();
    type State = IoPost;
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoPost {
    pub contents: String,
    pub donations_recipient : ActorId
}

