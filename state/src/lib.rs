#![no_std]

use gear_lib::non_fungible_token::{
    state::NFTQueryReply,
    token::{Token, TokenId},
};
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, Vec, prelude::*};
use program_io::ProgramMetadata;

// Funciones para poder leer el state con el metawasm.

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn info(state: State) -> String {
        // It need at most one func
    }
}

