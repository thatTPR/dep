use candid::types::ic_types::principal;
use ic_cdk;
mod org;
mod user;
use {org, user};

use ic_cdk_macros::*;
use ic_utils::Canister;

// TODO profile metadata

// TODO User mebership in organisations
// TODO can create repo canisters, tracks storage for them so it does not exceed allowance. By default two canisters are created (one is the repo controller, the other one holds the actual data (that is what is being tracked))

fn authenticate(id: String) { // TODO authenticate
}
fn createAccount() {}

use super::repo;
fn createRepo() {
    let builder = ic_utils::Canister::builder();

    super::repo
}
