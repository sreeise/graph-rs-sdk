use crate::drive::event::DownloadFormat;
use crate::drive::ItemResult;
use crate::prelude::MutateUrl;
use std::ffi::OsString;
use std::path::PathBuf;

pub trait MutateDownload {
    fn format(&mut self, format: DownloadFormat);
    fn rename(&mut self, file_name: OsString);

    fn format_glb(&mut self) {
        self.format(DownloadFormat::GLB);
    }

    fn format_html(&mut self) {
        self.format(DownloadFormat::HTML);
    }

    fn format_jpg(&mut self) {
        self.format(DownloadFormat::JPG);
    }

    fn format_pdf(&mut self) {
        self.format(DownloadFormat::PDF);
    }
}

pub trait IntoFetch {
    fn send(&mut self) -> ItemResult<PathBuf>;
}

pub trait IntoItem<T>: MutateUrl {
    fn send(&mut self) -> ItemResult<T>;
}
