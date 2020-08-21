use crate::{Row, RowValue};
use serde_json;
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub struct Table<T: RowValue> {
    basepath: PathBuf,
    _phantom: PhantomData<T>,
}

impl<T: RowValue> Table<T> {
    pub(crate) fn new<P: AsRef<Path>>(basepath: P) -> Self {
        Table {
            basepath: basepath.as_ref().to_path_buf(),
            _phantom: PhantomData,
        }
    }

    pub fn create<PK: AsRef<str>>(&mut self, pk: PK, value: T) -> Row<T> {
        let pk = pk.as_ref();
        let row_path = self.basepath.join(pk);
        let row = Row::new(row_path, value);
        row.write();
        row
    }

    pub fn read<PK: AsRef<str>>(&self, pk: PK) -> Option<Row<T>> {
        let pk = pk.as_ref();
        let row_path = self.basepath.join(format!("{}.json", pk));
        if !row_path.exists() || !row_path.is_file() {
            return None;
        }
        let data = fs::read(&row_path).ok()?;
        let value = serde_json::from_slice::<T>(&data).ok()?;
        Some(Row::new(row_path, value))
    }

    pub fn has<PK: AsRef<str>>(&self, pk: PK) -> bool {
        let pk = pk.as_ref();
        let row_path = self.basepath.join(format!("{}.json", pk));
        row_path.exists() && row_path.is_file()
    }

    pub fn read_all_pk<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        self.basepath
            .read_dir()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|f| {
                f.path()
                    .file_stem()
                    .and_then(|s| s.to_owned().into_string().ok())
            })
    }

    pub fn read_all<'a>(&'a self) -> impl Iterator<Item = Row<T>> + 'a {
        self.read_all_pk().filter_map(move |pk| self.read(pk))
    }

    pub fn update_if_exist<PK: AsRef<str>, F: Fn(&mut Row<T>)>(
        &mut self,
        pk: PK,
        modify: F,
    ) -> bool {
        if let Some(mut row) = self.read(pk) {
            modify(&mut row);
            row.write()
        } else {
            false
        }
    }

    pub fn delete<PK: AsRef<str>>(&mut self, pk: PK) -> bool {
        if let Some(row) = self.read(pk) {
            row.delete()
        } else {
            true
        }
    }
}
