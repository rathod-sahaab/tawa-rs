/// Unique identifier for a file or data item
pub type ItemId = u64;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FolderEntry {
    File(ItemId),
    Folder(usize), // index into FoldersCollection.folders
}

pub struct Folder {
    pub name: &'static str,
    pub entries: [Option<FolderEntry>; 16],
    pub count: usize,
}

impl Folder {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            entries: [None; 16],
            count: 0,
        }
    }
    pub fn add_entry(&mut self, entry: FolderEntry) -> bool {
        if self.count < self.entries.len() {
            self.entries[self.count] = Some(entry);
            self.count += 1;
            true
        } else {
            false
        }
    }
    pub fn remove_file(&mut self, id: ItemId) -> bool {
        let mut i = 0;
        while i < self.count {
            if self.entries[i] == Some(FolderEntry::File(id)) {
                let mut j = i;
                while j + 1 < self.count {
                    self.entries[j] = self.entries[j + 1];
                    j += 1;
                }
                self.entries[self.count - 1] = None;
                self.count -= 1;
                return true;
            }
            i += 1;
        }
        false
    }
    pub fn remove_folder(&mut self, folder_idx: usize) -> bool {
        let mut i = 0;
        while i < self.count {
            if self.entries[i] == Some(FolderEntry::Folder(folder_idx)) {
                let mut j = i;
                while j + 1 < self.count {
                    self.entries[j] = self.entries[j + 1];
                    j += 1;
                }
                self.entries[self.count - 1] = None;
                self.count -= 1;
                return true;
            }
            i += 1;
        }
        false
    }
}
