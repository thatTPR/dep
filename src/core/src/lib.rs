use ic_cdk::storage as cdk_storage;
use {fileSystem, identity, repo};

use ic_utils;

use ic_cdk_macros::*;
use ic_utils::canister;
use serde::{Serialize, Deserialize} ;

#[ic_cdk_macros::heartbeat]
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








#[query(name = "accessIdentity")]
fn accesIdentity(principal: candid::Principal) -> (String, bool){

}

#[update(name = "createIdentity")]
fn createIdentity(principal: candid::Principal, text: String) -> String{
   
    let idCanister = createID(principal, text) ;
    registerId(idCanister)
}
#[update(name = "modifyIdentity")]
fn modifyIdentity(principal: candid::Principal, text: String) {

}
#[update(name = "deleteIdentity")]
fn deleteIdentiy() ->String{
    
}

#[query(name = "search")]
fn search(){

}
