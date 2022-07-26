mod deps;
mod issues;
mod contributors; 

use {deps, issues, contributors};

use ic_cdk::storage::*;
use ic_cdk_macros::{import, init};

#[import(canister = "repoContent")]
struct repoContent;



// TODO by default two canisters are created for each repo ( controller + content) Controller holds metadata , Allocated space is tracked and given back to the principal
use candid:: // TODO import repoContent and identity

struct Repo {
    id: String,
    public: bool ,
    // content
    issues: issues::Issues,
    contributors: contributors::Contributors,
    langs: Vec<String>, // TODO
    pr: Vec<pr>
    
    // ops:
    // Deps
}
