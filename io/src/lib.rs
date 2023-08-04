#![no_std]

use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::NFTState,
    token::*,
};
use gmeta::{In, InOut, Out, Metadata};
use gstd::{prelude::*, ActorId};

// pub use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;
use primitive_types::H256;

pub struct ProgramMetadata;

// Aqui unicamente se pone las estructuras y enums que se
// usaran en el smart contractm asi como la metadatad.

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = ();
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}
