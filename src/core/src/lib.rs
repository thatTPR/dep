use ic_cdk::storage as cdk_storage;
use {fileSystem, identity, repo};

use ic_utils;

use ic_cdk_macros::*;
use ic_utils::canister;
use serde::{Serialize, Deserialize} ;

use candid::utils:: ; // TODO import core-Extended and References to identities  and References to repositories( securely and held in active memory)

#[ic_cdk_macros::heartbeat]


// TODO make network cheaper as it grows and more canisters get created 


fn heartbeat() {}

pub fn createId(principal: candid::Principal , text: String) -> ic_utils::Canister { // TODO
    
}

pub fn registerId(canister: ic_utils::canister::Canister){ // TODO
    let reader  = ic_cdk::api::stable::StableReader::default() ;
    serde::Deserializer::deserialize_struct(self, "", fields, visitor)
    let writer = ic_cdk::api::stable::StableWriter ; 
    reader = 
}
pub fn updateId(canister: ic_utils::Canister::Canister){// TODO

}
pub fn provideId(principal: candid::Principal) -> ic_utils::canister::Canister{ // TODO
    ic_cdk::storage::get()
}

// TODO handle memory limits



#[query(name = "accessGuest")]
fn accessGuest(){
    // TODO returns a basic session instance and 
}

#[query(name = "accessIdentity")]
fn accesIdentity(principal: candid::Principal) -> (String, bool){
    // TODO return session instance for the Internet Identity ; 
    
}

#[update(name = "createIdentity")]
fn createIdentity(principal: candid::Principal, text: String) -> (String, bool ){
    // Registers Identity in stable storage and returns a session instance
    let idCanister = createID(principal, text) ;
    registerId(idCanister)
    accessIdentity(principal)
}
#[update(name = "modifyIdentity")]
fn modifyIdentity(principal: candid::Principal, text: String) {

}
#[update(name = "deleteIdentity")]
fn deleteIdentiy(principal: Candid::Principal , confirmation: bool) ->String{
    //TODO unregisters identity and deletes all associated data from storage  , kills all associated canisters bottom up with checks to insure success
}
mod search ;
use search ;
#[query(name = "search")]
fn Search(search: String){
    // Repo Search format :: repo ; topic:* org:* *
    // In repo search format 
}



