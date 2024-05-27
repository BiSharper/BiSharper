use std::ops::{Deref, DerefMut};
use rfsa::*;
use rfsa::impls::memory::MemoryFileSystem;
use rfsa::macros::VMeta;

pub type GameFile = VFile<GameMeta>;
pub type ReadableGameFile = ReadableVFile<GameMeta>;
pub type WritableGameFile<'a> = WritableVFile<'a, GameMeta, GameFileSystem>;
pub type ReadableGameMetadata = ReadableVMetadata<GameMeta>;
pub type WritableGameMetadata<'a> = WritableVMetadata<'a, GameMeta, GameFileSystem>;
pub type GameDirectory<'a> = VDirectory<'a, GameMeta, GameFileSystem>;

#[derive(VMeta, Copy, Clone, Default, Eq, PartialEq)]
pub struct GameMeta {

}

pub struct GameFileSystem { file_system: MemoryFileSystem<GameMeta> }

impl Deref for GameFileSystem {
    type Target = MemoryFileSystem<GameMeta>;

    fn deref(&self) -> &Self::Target { &self.file_system }
}

impl DerefMut for GameFileSystem {
    fn deref_mut(&mut self) -> &mut Self::Target  { &mut self.file_system }
}

//TODO: Bank/GFS Interop
// #[cfg(feature = "bank-filesystem")]
// impls Into<GameMeta> for crate::bank::BankFileMeta {
//     fn into(self) -> GameMeta {
//         GameMeta {
//
//         }
//     }
// }
//
// #[cfg(feature = "bank-filesystem")]
// impls GameFileSystem {
//     pub fn bank_import_file(&mut self, archive: &crate::bank::BankArchive, path: &VPath) -> Result<Option<GameFile>> {
//         todo!()
//     }
//
//     pub fn bank_import_file_advanced(&mut self, archive: &crate::bank::BankArchive, path: &VPath) -> Result<Option<GameFile>> {
//         todo!()
//     }
//
//     pub fn bank_import_archive_advanced(&mut self, archive: &crate::bank::BankArchive) -> Result<Vec<(VPath, GameFile)>> {
//         todo!()
//     }
//
// }