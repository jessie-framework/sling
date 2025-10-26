pub use bincode::{Decode, Encode};

use sling_globals::GLOBALS;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;

pub trait Cached: Encode + Decode<()> + Hash {
    /// Looks for the compilers cache if the struct exists.  If it does, turns the struct into whatever was in the cache.
    fn get_link(&self) -> Link {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        Link(hasher.finish())
    }
    fn try_load(&mut self) {
        if let Some(file) = self.file() {
            match std::fs::read(&file) {
                Ok(v) => {
                    let decoded: Result<(Self, _), _> =
                        bincode::decode_from_slice(&v, bincode::config::standard());
                    match decoded {
                        Ok((v, _)) => *self = v,
                        Err(_) => {
                            std::fs::remove_file(file);
                        }
                    }
                }
                Err(_) => {
                    // prob should insert some logic
                }
            }
        }
    }
    fn file(&mut self) -> Option<PathBuf> {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();
        let cache_dir = GLOBALS.cache.as_ref();
        match cache_dir {
            Some(path) => Some(path.join(format!("{}.slc", hash))),
            _ => None,
        }
    }

    fn upload(&self, link: Link) {
        match GLOBALS.cache.as_ref() {
            None => {}
            Some(dir) => {
                std::fs::create_dir_all(dir).unwrap();
                let file_dir = dir.join(format!("{}.slc", link.0));
                let mut file = File::create(file_dir).unwrap();
                let _ =
                    bincode::encode_into_std_write(self, &mut file, bincode::config::standard());
            }
        }
    }
}
#[derive(Decode, Encode, Hash, Copy, Clone)]
pub struct Link(u64);
