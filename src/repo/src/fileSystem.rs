mod fileHandle {
    use ic_cdk::api::stable;
    use ic_cdk::storage;

    fn newFile() {
        storage::get();
    }
}
mod folderHandle {
    use super;
    use super;
    use ic_cdk::storage;

    fn newFolder() {
        storage::get();
    }
}

use fileHandle;
use folderHandle;

use ic_cdk::api::stable as Stable;
use ic_cdk::export::Principal;
use ic_cdk::storage;

use std::string::*;
use std::vec::Vec;

struct Folder {
    Files: std::vec::Vec<File>,
    folders: Std::vec::Vec<Folder>,
    start: u32,
    end: u32,
    principal: ic_cdk::export::Principal,
}

struct File {
    name: std::string::String,
    Data: std::string::String,
    start: u32,
    end: u32,
}
struct Repo {
    folder: Folder,
}
impl Repo {
    fn new(Folder: Folder) -> Self {
        Self {
            folder: stable::StableWriter(Folder),
        }
    }
    fn new() -> Self {
        Self { folder: Null }
    }
    fn addMain(folder: Folder) {
        self.folder = folder;
    }
    fn addFolder() {}
}

use ic_cdk::storage;
// TODO file new
// TODO write
// TODO read
// TODO find in file
// TODO find file

// TODO folder
// TODO folder read
// TODO folder write
// TODO folder new

// TODO private
// TODO public

// TODO owner for sys
// Each repository instantiates a new filesystem namespace and then everything is handled through the git proxy
