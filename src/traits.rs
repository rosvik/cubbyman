use clio::Input;
use std::path::PathBuf;

pub trait ToPath {
    fn to_path_buf(&self) -> PathBuf;
}
impl ToPath for Input {
    fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(self.to_string())
    }
}
