use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::HashMap;
use primitive_types::{H256, U256};

// Aqui tienes que agregar los nombres de tus structs como enums
// con los que trabajaras.
use program_io::{ProgramMetadata};

// Aqui cambias el tipo de tu estado.
static mut STATENAME: Option<TYPE> = None;

/*
Esta funcion te retorna tu state mutable, solo hay que cambiar
los nombres del tipo del STATE junto con el nombre del state
en si.
*/
fn state_mut() -> &'static mut TYPEOFRETURN {
    let state = unsafe { STATENAME.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

// Funcionoes necesarias para que funcine, no se necesita la funcion
// de metahash

#[no_mangle]
unsafe extern "C" fn init() {
}

#[no_mangle]
unsafe extern "C" fn handle() {
}

#[no_mangle]
extern "C" fn state() {
}