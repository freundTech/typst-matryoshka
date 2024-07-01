use comemo::Prehashed;
use typst::{Library, World};
use typst::diag::{FileError, FileResult};
use typst::eval::Tracer;
use typst::foundations::{Bytes, Datetime, Smart};
use typst::layout::PageElem;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::visualize::Luma;
use wasm_minimal_protocol::*;

use crate::font::FontLoader;

mod font;

initiate_protocol!();

#[wasm_func]
pub fn compile(byte_source: &[u8]) -> Vec<u8> {
    let text = String::from_utf8(byte_source.to_vec()).unwrap();

    let world = MatryoshkaWorld::new(text);

    let mut tracer = Tracer::new();
    let result = typst::compile(&world, &mut tracer);

    match result {
        Ok(document) => {
            let svg = typst_svg::svg(&document.pages[0].frame);

            return Vec::from(svg.as_bytes())
        }
        Err(_errors) => {
            panic!("Failed");
        }
    }
}


struct MatryoshkaWorld {
    library: Prehashed<Library>,
    source: Source,
    book: Prehashed<FontBook>,
    fonts: Vec<Font>,
}

impl MatryoshkaWorld {
    pub fn new(text: String) ->  Self {
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

    fn source(&self, _id: FileId) -> FileResult<Source> {
        Err(FileError::Other(Some("Matryoshka doesn't support imports".into())))
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::Other(Some("Matryoshka doesn't support imports".into())))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Datetime::from_ymd(1970, 1, 1)
    }
}