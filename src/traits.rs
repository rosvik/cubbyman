use clio::Input;
use std::path::{Path, PathBuf};

pub trait ToString {
    fn to_string(&self) -> String;
}
impl ToString for PathBuf {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

pub trait ToRelative {
    fn to_relative(&self, base: &Path) -> PathBuf;
}
impl ToRelative for PathBuf {
    fn to_relative(&self, base: &Path) -> PathBuf {
        base.join(self)
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
