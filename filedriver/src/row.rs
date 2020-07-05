use serde_json;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub trait RowValue
where
    Self: serde::Serialize,
    Self: serde::de::DeserializeOwned,
{
}

impl<T> RowValue for T
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
}

pub struct Row<T: RowValue> {
    path: PathBuf,
    value: T,
}

impl<T: RowValue> Deref for Row<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: RowValue> DerefMut for Row<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: RowValue> Row<T> {
    pub(crate) fn new<P: AsRef<Path>>(path: P, value: T) -> Self {
        Row {
            path: path.as_ref().to_owned(),
            value,
        }
    }
    pub fn write(&self) -> bool {
        let contents = match serde_json::to_vec(&self.value) {
            Ok(v) => v,
            Err(_) => return false,
        };
        fs::write(&self.path, contents).is_ok()
    }
    pub fn delete(self) -> bool {
        fs::remove_file(&self.path).is_ok()
    }

    pub fn take(self) -> T {
        self.value
    }
}
