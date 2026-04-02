use clio::Input;
use std::path::PathBuf;

pub trait ToString {
    fn to_string(&self) -> String;
}
impl ToString for PathBuf {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

pub trait ToAbsolute {
    fn to_absolute(&self) -> PathBuf;
}
impl ToAbsolute for PathBuf {
    fn to_absolute(&self) -> PathBuf {
        if self.is_absolute() {
            return self.clone();
        }
        let path = std::env::current_dir().unwrap().join(self);
        path.canonicalize().unwrap_or_else(|e| {
            panic!("Failed to resolve path '{}': {}", path.to_string_lossy(), e)
        })
    }
}

pub trait ToPath {
    fn to_path_buf(&self) -> PathBuf;
}
impl ToPath for Input {
    fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(self.to_string())
    }
}
