use std::collections::HashMap;
use std::path::PathBuf;
use comemo::Prehashed;
use typst::{Library, World};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Smart};
use typst::layout::PageElem;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::visualize::Luma;

use crate::font::FontLoader;
pub(crate) struct MatryoshkaWorld {
    library: Prehashed<Library>,
    source: Source,
    book: Prehashed<FontBook>,
    fonts: Vec<Font>,
    filesystem: HashMap<String, Vec<u8>>,
}

impl MatryoshkaWorld {
    pub fn new(text: String, filesystem: HashMap<String, Vec<u8>>) ->  Self {
        let mut library = Library::builder().build();
        let background_property = PageElem::set_fill(Some(Luma::new(1f32, 1f32).into()));
        let height_property = PageElem::set_height(Smart::Auto);
        library.styles.apply_one(background_property.into());
        library.styles.apply_one(height_property.into());

        let virtual_path = VirtualPath::new("main.typ");
        let file_id = FileId::new(None, virtual_path);
        let source = Source::new(file_id, text);

        let mut font_loader = FontLoader::new();
        font_loader.load();

        MatryoshkaWorld {
            library: Prehashed::new(library),
            source: source,
            book: Prehashed::new(font_loader.book),
            fonts: font_loader.fonts,
            filesystem: filesystem,
        }
    }
}

impl World for MatryoshkaWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn main(&self) -> Source {
        self.source.clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let path = match id.vpath().as_rootless_path().as_os_str().to_str() {
            Some(path) => path,
            None => return Err(FileError::InvalidUtf8)
        };
        match self.filesystem.get(path) {
            Some(ok) => Ok(Source::new(id, String::from_utf8(ok.clone()).unwrap())),
            None => Err(FileError::NotFound(PathBuf::from(path)))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = match id.vpath().as_rootless_path().as_os_str().to_str() {
            Some(path) => path,
            None => return Err(FileError::InvalidUtf8)
        };
        match self.filesystem.get(path) {
            Some(bytes) => Ok(bytes.clone().into()),
            None => Err(FileError::NotFound(PathBuf::from(path)))
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Datetime::from_ymd(1970, 1, 1)
    }
}
