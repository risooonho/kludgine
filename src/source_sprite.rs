use super::{
    math::Point,
    math::Rect,
    scene::{Element, Scene},
    sprite::RenderedSprite,
    texture::Texture,
    KludgineHandle,
};
#[derive(Clone)]
pub struct SourceSprite {
    pub(crate) handle: KludgineHandle<SourceSpriteData>,
}

pub(crate) struct SourceSpriteData {
    pub location: Rect<u32>,
    pub texture: Texture,
}

impl SourceSprite {
    pub fn new(location: Rect<u32>, texture: Texture) -> Self {
        SourceSprite {
            handle: KludgineHandle::new(SourceSpriteData { location, texture }),
        }
    }

    pub fn entire_texture(texture: Texture) -> Self {
        let (w, h) = {
            let texture = texture.handle.read().expect("Error reading source sprice");
            (texture.image.width(), texture.image.height())
        };
        Self::new(Rect::sized(0, 0, w, h), texture)
    }

    pub fn render_at(&self, scene: &mut Scene, location: Point) {
        let (w, h) = {
            let source = self.handle.read().expect("Error locking source_sprite");
            (source.location.width(), source.location.height())
        };
        scene.elements.push(Element::Sprite(RenderedSprite::new(
            Rect::sized(location.x, location.y, w as f32, h as f32),
            self.clone(),
        )));
    }
}
