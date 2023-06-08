#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId,Encode,Decode,TypeInfo,Debug};

pub struct ChronicleMetadata;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum PostAction{
    CreatePost(String, String),
    DonateToPoster(u32)
}

impl Metadata for ChronicleMetadata{
    type Init = ();
    type Handle = InOut<PostAction, Vec<IoPost>>;
    type Reply = ();
    type Others =();
    type Signal = ();
    type State = Vec<IoPost>;
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoPost {
    pub title: String,
    pub body: String,
    pub poster_wallet : ActorId
}

