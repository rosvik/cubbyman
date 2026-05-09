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
        self.canonicalize().unwrap_or_else(|e| {
            panic!("Failed to resolve path '{}': {}", self.to_string_lossy(), e)
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
    use std::env::temp_dir;
    use std::fs::create_dir_all;

    #[test]
    fn test_to_relative() {
        let path = PathBuf::from("dir/test.txt");
        assert_eq!(
            path.to_relative(&PathBuf::from("tests")),
            PathBuf::from("tests/dir/test.txt")
        );
    }

    #[test]
    fn test_to_absolute_resolves_dotdot() {
        let tmp = temp_dir().canonicalize().unwrap();
        let dir = tmp.join("dotdot_test");
        create_dir_all(&dir).unwrap();

        // `dotdot_test/..` should canonicalize back to `tmp`
        assert_eq!(dir.join("..").to_absolute(), tmp);

        let _ = std::fs::remove_dir(&dir);
    }

    #[test]
    fn test_to_absolute_from_relative() {
        let current_dir = std::env::current_dir().unwrap();
        let path = PathBuf::from(".");
        assert_eq!(path.to_absolute(), current_dir);
    }
}
