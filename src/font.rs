use typst::text::{Font, FontBook};

/// Searches for fonts.
pub struct FontLoader {
    /// Metadata about all discovered fonts.
    pub book: FontBook,
    /// Slots that the fonts are loaded into.
    pub fonts: Vec<Font>,
}

impl FontLoader {
    /// Create a new, empty system searcher.
    pub fn new() -> Self {
        Self { book: FontBook::new(), fonts: vec![] }
    }

    /// Load embedded fonts
    pub fn load(&mut self) {
        for data in typst_assets::fonts() {
            let buffer = typst::foundations::Bytes::from_static(data);
            for font in Font::iter(buffer) {
                self.book.push(font.info().clone());
                self.fonts.push(font);
            }
        }
    }
}
