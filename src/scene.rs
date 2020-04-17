use super::{
    math::{Point, Rect, Size},
    timing::Moment,
    KludgineHandle, KludgineResult,
};
use image::{DynamicImage, RgbaImage};
use rgx::core::BindingGroup;

use crossbeam::atomic::AtomicCell;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    time::Duration,
};
use winit::event::VirtualKeyCode;

pub struct Scene {
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub(crate) scale_factor: f32,
    pub(crate) size: Size,
    pub(crate) sprites: Vec<Sprite>,
    now: Option<Moment>,
    elapsed: Option<Duration>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scale_factor: 1.0,
            size: Size::default(),
            pressed_keys: HashSet::new(),
            now: None,
            elapsed: None,
            sprites: Vec::new(),
        }
    }

    pub(crate) fn start_frame(&mut self) {
        let last_start = self.now;
        self.now = Some(Moment::now());
        self.elapsed = match last_start {
            Some(last_start) => self.now().checked_duration_since(&last_start),
            None => None,
        };
        self.sprites.clear();
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn now(&self) -> Moment {
        self.now.expect("now() called without starting a frame")
    }

    pub fn elapsed(&self) -> Option<Duration> {
        self.elapsed
    }

    pub fn is_initial_frame(&self) -> bool {
        self.elapsed.is_none()
    }

    pub fn render_sprite_at(&mut self, source_sprite: &SourceSprite, location: Point) {
        let (w, h) = {
            let source = source_sprite
                .handle
                .read()
                .expect("Error locking source_sprite");
            (source.location.width(), source.location.height())
        };
        self.sprites.push(Sprite::new(
            Rect::sized(location.x, location.y, w, h),
            source_sprite.clone(),
        ));
    }

    // pub fn get(&self, id: Entity) -> Option<Mesh> {
    //     match self.world.get_component::<MeshHandle>(id) {
    //         Some(handle) => Some(Mesh {
    //             id,
    //             handle: handle.as_ref().clone(),
    //         }),
    //         None => None,
    //     }
    // }

    // pub fn cached_mesh<S: Into<String>, F: FnOnce(&mut Scene2d) -> KludgineResult<Mesh>>(
    //     &mut self,
    //     name: S,
    //     initializer: F,
    // ) -> KludgineResult<Mesh> {
    //     let name = name.into();
    //     match self.lazy_mesh_cache.get(&name) {
    //         Some(mesh) => Ok(mesh.clone()),
    //         None => {
    //             let new_mesh = initializer(self)?;
    //             self.lazy_mesh_cache.insert(name, new_mesh.clone());
    //             Ok(new_mesh)
    //         }
    //     }
    // }

    // pub fn create_mesh<M: Into<Material>>(&mut self, shape: Shape, material: M) -> Mesh {
    //     let material = material.into();
    //     let storage = KludgineHandle::wrap(MeshStorage {
    //         shape,
    //         material,
    //         angle: Rad(0.0),
    //         scale: 1.0,
    //         position: Point2d::new(0.0, 0.0),
    //         children: HashMap::new(),
    //     });
    //     let handle = MeshHandle { storage };
    //     let id = self.world.insert((), vec![(handle.clone(),)])[0];
    //     Mesh { id, handle }
    // }
}

#[derive(Default)]
pub(crate) struct Frame {
    pub started_at: Option<Moment>,
    pub updated_at: Option<Moment>,
    pub size: Size,
    pub commands: Vec<FrameCommand>,
    pub(crate) textures: HashMap<u64, LoadedTexture>,
}

impl Frame {
    pub fn update(&mut self, scene: &Scene) {
        self.started_at = Some(scene.now());
        self.commands.clear();

        self.size = scene.size;

        let mut referenced_texture_ids = HashSet::new();

        let mut current_texture_id: Option<u64> = None;
        let mut current_batch: Option<SpriteBatch> = None;
        for sprite_handle in scene.sprites.iter() {
            let sprite = sprite_handle
                .handle
                .read()
                .expect("Error locking sprite for update");
            let source = sprite
                .source
                .handle
                .read()
                .expect("Error locking source for update");
            let texture = source
                .texture
                .handle
                .read()
                .expect("Error locking texture for update");

            if current_texture_id.is_none() || current_texture_id.as_ref().unwrap() != &texture.id {
                if let Some(current_batch) = current_batch {
                    self.commands
                        .push(FrameCommand::DrawBatch(KludgineHandle::new(current_batch)));
                }
                current_texture_id = Some(texture.id);
                referenced_texture_ids.insert(texture.id);

                // Load the texture if needed
                let loaded_texture_handle = self
                    .textures
                    .entry(texture.id)
                    .or_insert(LoadedTexture::new(source.texture.clone()));
                let loaded_texture = loaded_texture_handle
                    .handle
                    .read()
                    .expect("Error locking loaded_texture");
                if loaded_texture.binding.is_none() {
                    self.commands
                        .push(FrameCommand::LoadTexture(loaded_texture_handle.clone()));
                }

                current_batch = Some(SpriteBatch::new(loaded_texture_handle.clone()));
            }

            let current_batch = current_batch.as_mut().unwrap();
            current_batch.sprites.push(sprite_handle.clone());
        }

        if let Some(current_batch) = current_batch {
            self.commands
                .push(FrameCommand::DrawBatch(KludgineHandle::new(current_batch)));
        }

        let dead_texture_ids = self
            .textures
            .keys()
            .filter(|id| !referenced_texture_ids.contains(id))
            .map(|id| *id)
            .collect::<Vec<_>>();
        for id in dead_texture_ids {
            self.textures.remove(&id);
        }

        self.updated_at = Some(Moment::now());
    }
}

pub(crate) enum FrameCommand {
    LoadTexture(LoadedTexture),
    DrawBatch(KludgineHandle<SpriteBatch>),
}

lazy_static! {
    static ref TEXTURE_ID_CELL: AtomicCell<u64> = { AtomicCell::new(0) };
}

#[derive(Clone)]
pub struct Texture {
    pub(crate) handle: KludgineHandle<TextureData>,
}

pub(crate) struct TextureData {
    pub id: u64,
    pub image: RgbaImage,
}

impl Texture {
    pub fn new(image: DynamicImage) -> Self {
        let image = image.to_rgba();
        let id = TEXTURE_ID_CELL.fetch_add(1);
        Self {
            handle: KludgineHandle::new(TextureData { id, image }),
        }
    }

    pub fn load<P: AsRef<Path>>(from_path: P) -> KludgineResult<Self> {
        let img = image::open(from_path)?;

        Ok(Self::new(img))
    }
}

#[derive(Clone)]
pub struct LoadedTexture {
    pub(crate) handle: KludgineHandle<LoadedTextureData>,
}

pub(crate) struct LoadedTextureData {
    pub texture: Texture,
    pub(crate) binding: Option<BindingGroup>,
}

impl LoadedTexture {
    pub fn new(texture: Texture) -> Self {
        LoadedTexture {
            handle: KludgineHandle::new(LoadedTextureData {
                texture,
                binding: None,
            }),
        }
    }
}

#[derive(Clone)]
pub struct SourceSprite {
    pub(crate) handle: KludgineHandle<SourceSpriteData>,
}

pub(crate) struct SourceSpriteData {
    pub location: Rect,
    pub texture: Texture,
}

impl SourceSprite {
    pub fn new(location: Rect, texture: Texture) -> Self {
        SourceSprite {
            handle: KludgineHandle::new(SourceSpriteData { location, texture }),
        }
    }

    pub fn entire_texture(texture: Texture) -> Self {
        let (w, h) = {
            let texture = texture.handle.read().expect("Error reading source sprice");
            (texture.image.width() as f32, texture.image.height() as f32)
        };
        Self::new(Rect::sized(0.0, 0.0, w, h), texture)
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub(crate) handle: KludgineHandle<SpriteData>,
}

impl Sprite {
    pub fn new(render_at: Rect, source: SourceSprite) -> Self {
        Self {
            handle: KludgineHandle::new(SpriteData { render_at, source }),
        }
    }
}

pub(crate) struct SpriteData {
    pub render_at: Rect,
    pub source: SourceSprite,
}

pub(crate) struct SpriteBatch {
    pub loaded_texture: LoadedTexture,
    pub sprites: Vec<Sprite>,
}

impl SpriteBatch {
    pub fn new(loaded_texture: LoadedTexture) -> Self {
        SpriteBatch {
            loaded_texture,
            sprites: Vec::new(),
        }
    }
}
