mod deps;

mod fileSystem;
mod git_proxy;
mod issues;

use {deps, fileSystem, issues};

use ic_cdk::storage::*;

// TODO by default two canisters are created for each repo ( controller + content) Controller holds metadata , Allocated space is tracked and given back to the principal

// TOD
struct Repo {
    // id:
    // fileSys: fileSystem:
    // issues: issues:
    // Contributors:
    // Languages:
    // pullReqs:
    // ops:
    // Depends:
}
