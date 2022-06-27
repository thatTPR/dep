use candid::types::ic_types::principal;
use ic_cdk;
mod org;
mod user;

use {guest, org, user};

use ic_cdk_macros::*;
use ic_utils::Canister;

use candid:: ;// Import core and Repo, Store references to repositories 
// Default guest

struct Identity {}
// TODO profile metadata

// TODO User mebership in organisations
// TODO can create repo canisters, tracks storage for them so it does not exceed allowance. By default two canisters are created (one is the repo controller, the other one holds the actual data (that is what is being tracked))
// TODO token canister as well associated , gets dependency information and executes rewards
fn authenticate(id: String) { // TODO authenticate
}
fn createAccount() {}

use super::repo;
fn createRepo() {
    let builder = ic_utils::Canister::builder();

    super::repo
}
