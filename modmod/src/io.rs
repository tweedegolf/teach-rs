use std::{
    fmt,
    fs::{self, File},
    io,
    path::Path,
};

use error_stack::{Context, IntoReport, Result, ResultExt};
use fs_extra::dir::DirContent;

pub trait PathExt {
    fn create_dir_all<C: Context + Default>(&self) -> Result<(), C>;
    fn read_to_string<C: Context + Default>(&self) -> Result<String, C>;
    fn create_file<C: Context + Default>(&self) -> Result<File, C>;
    fn get_dir_content<C: Context + Default>(&self) -> Result<DirContent, C>;
    fn copy<C: Context + Default>(&self, to: impl AsRef<Path>) -> Result<(), C>;
}

impl<T: AsRef<Path>> PathExt for T {
    fn create_dir_all<C: Context + Default>(&self) -> Result<(), C> {
        let path = self.as_ref();
        fs::create_dir_all(path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error creating directory at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context(C::default())
    }

    fn read_to_string<C: Context + Default>(&self) -> Result<String, C> {
        let path = self.as_ref();

        fs::read_to_string(path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error reading file at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context(C::default())
    }

    fn create_file<C: Context + Default>(&self) -> Result<File, C> {
        let path = self.as_ref();

        File::create(path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error creating file at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context(C::default())
    }

    fn get_dir_content<C: Context + Default>(&self) -> Result<DirContent, C> {
        let path = self.as_ref();
        fs_extra::dir::get_dir_content(path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error getting contents of directory at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context(C::default())
    }

    fn copy<C: Context + Default>(&self, to: impl AsRef<Path>) -> Result<(), C> {
        let from = self.as_ref();
        let to = to.as_ref();
        fs::copy(from, to)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error copying file from {from} to {to}",
                    from = from.to_string_lossy(),
                    to = to.to_string_lossy()
                )
            })
            .change_context(C::default())?;
        Ok(())
    }
}

pub trait WriteExt {
    fn write_fmt<C: Context + Default>(&mut self, fmt: fmt::Arguments) -> Result<(), C>;
    fn write_all<C: Context + Default>(&mut self, content: impl AsRef<[u8]>) -> Result<(), C>;
}

impl<W: io::Write> WriteExt for W {
    fn write_fmt<C: Context + Default>(&mut self, fmt: fmt::Arguments) -> Result<(), C> {
        io::Write::write_fmt(self, fmt)
            .into_report()
            .change_context(C::default())
    }

    fn write_all<C: Context + Default>(&mut self, content: impl AsRef<[u8]>) -> Result<(), C> {
        io::Write::write_all(self, content.as_ref())
            .into_report()
            .change_context(C::default())
    }
}

pub fn copy_files<P: AsRef<Path>, C: Context + Default>(files: &[P], dest: &Path) -> Result<(), C> {
    files
        .iter()
        .filter_map(|path| path.as_ref().file_name().map(|name| (path, name)))
        .try_for_each(|(path, name)| path.copy(dest.join(name)))
}
