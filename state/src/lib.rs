
#![no_std]


use gstd::{ActorId, Vec, prelude::*};


pub mod metafns {
    use chronicles_io::ChronicleMetadata;
    use gmeta::Metadata;
    pub type State = <ChronicleMetadata as Metadata>::State;

    pub fn foo(state : State) -> u32{
        42
    }


}
