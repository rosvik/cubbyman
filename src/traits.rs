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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_relative() {
        let path = PathBuf::from("dir/test.txt");
        assert_eq!(
            path.to_relative(&PathBuf::from("tests")),
            PathBuf::from("tests/dir/test.txt")
        );
    }
}
