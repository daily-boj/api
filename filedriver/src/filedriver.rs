use crate::{RowValue, Table};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Filedriver {
    basepath: PathBuf,
}

impl Filedriver {
    pub fn connect<P: AsRef<Path>>(basepath: P) -> io::Result<Filedriver> {
        let basepath = basepath.as_ref();
        fs::create_dir_all(basepath)?;
        Ok(Filedriver {
            basepath: basepath.to_owned(),
        })
    }

    pub fn table<T: RowValue, Name: AsRef<str>>(&self, name: Name) -> Table<T> {
        let name = name.as_ref();
        let column_basepath = self.basepath.join(name);
        Table::new(column_basepath)
    }
}
