mod git_proxy ;
mod fileSystem;
mod git ;
use {git_proxy, fileSystem, git};

use candid:: ;
// Import repo

use ic_utils::
struct Repo {
    owner: // Repo owner (cansister type - identity)
    father: // Repo parent (canister type - repo)
    next: ic,// Next content canister, Not storing prev because it is not necesarry everything is simply given to the User actor
    fileSys: fileSystem::Repo,
    git: git_proxy::,
   
}