mod git_proxy ;
mod fileSystem;
mod git ;
use {git_proxy, fileSystem, git};

// This holds the filesystem for the repo
struct Repo {
    owner: // Repo owner (cansister type - identity)
    father: // Repo parent (canister type - repo)
    next: ic,// Next content canister, Not storing prev because it is not necesarry everything is simply given to the User actor
    fileSys: fileSystem::Repo,
    git: git_proxy::Repo
   
}