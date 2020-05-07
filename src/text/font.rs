use crate::KludgineHandle;
use crossbeam::atomic::AtomicCell;
use lazy_static::lazy_static;
use rgx::core::*;
use rusttype::{gpu_cache, Scale};
lazy_static! {
    static ref GLOBAL_ID_CELL: AtomicCell<u64> = { AtomicCell::new(0) };
}

/// Font provides TrueType Font rendering
#[derive(Clone, Debug)]
pub struct Font {
    pub(crate) handle: KludgineHandle<FontData>,
}

impl Font {
    pub fn try_from_bytes(bytes: &'static [u8]) -> Option<Font> {
        let font = rusttype::Font::try_from_bytes(bytes)?;
        Some(Font {
            handle: KludgineHandle::new(FontData {
                font,
                id: GLOBAL_ID_CELL.fetch_add(1),
            }),
        })
    }

    pub async fn id(&self) -> u64 {
        let font = self.handle.read().await;
        font.id
    }

    pub async fn metrics(&self, size: f32) -> rusttype::VMetrics {
        let font = self.handle.read().await;
        font.font.v_metrics(rusttype::Scale::uniform(size))
    }

    pub async fn family(&self) -> Option<String> {
        let font = self.handle.read().await;
        match &font.font {
            rusttype::Font::Ref(f) => f.family_name(),
            _ => None,
        }
    }

    pub async fn weight(&self) -> ttf_parser::Weight {
        let font = self.handle.read().await;
        match &font.font {
            rusttype::Font::Ref(f) => f.weight(),
            _ => ttf_parser::Weight::Normal,
        }
    }

    pub async fn glyph(&self, c: char) -> rusttype::Glyph<'static> {
        let font = self.handle.read().await;
        font.font.glyph(c)
    }

    pub async fn pair_kerning(&self, size: f32, a: rusttype::GlyphId, b: rusttype::GlyphId) -> f32 {
        let font = self.handle.read().await;
        font.font.pair_kerning(Scale::uniform(size), a, b)
    }
}

#[derive(Debug)]
pub(crate) struct FontData {
    pub(crate) id: u64,
    pub(crate) font: rusttype::Font<'static>,
}

#[derive(Clone)]
pub(crate) struct LoadedFont {
    pub handle: KludgineHandle<LoadedFontData>,
}

impl LoadedFont {
    pub fn new(font: &Font) -> Self {
        Self {
            handle: KludgineHandle::new(LoadedFontData {
                font: font.clone(),
                cache: gpu_cache::Cache::builder().dimensions(512, 512).build(),
                binding: None,
                texture: None,
            }),
        }
    }
}

pub(crate) struct LoadedFontData {
    pub font: Font,
    pub cache: gpu_cache::Cache<'static>,
    pub(crate) binding: Option<BindingGroup>,
    pub(crate) texture: Option<rgx::core::Texture>,
}