#![no_std]
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use chronicles_io::*;

#[metawasm]
pub mod metafns {
    pub type State = IoChronicle;

    pub fn title(state: State) -> String{
        state.title;
    }

    pub fn body(state: State) -> String{
        state.body;
    }

    pub poster_wallet(state: State) -> ActorId{
        state.poster_wallet;
    }
}