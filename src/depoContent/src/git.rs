use super::fileSystem;
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
