use crate::{
    math::{Rect, Size},
    source_sprite::SourceSprite,
    sprite::Sprite,
    ui::{Component, Context, InteractiveComponent, Layout, SceneContext, StyledContext},
    KludgineResult,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Image {
    sprite: Sprite,
    current_frame: Option<SourceSprite>,
    options: ImageOptions,
}

#[derive(Debug)]
pub enum ImageScaling {
    AspectFit,
    AspectFill,
    Fill,
}

#[derive(Debug, Default)]
pub struct ImageOptions {
    pub scaling: Option<ImageScaling>,
}

#[derive(Debug, Clone)]
pub enum ImageCommand {
    SetSprite(Sprite),
    SetTag(Option<String>),
}

#[async_trait]
impl InteractiveComponent for Image {
    type Message = ();
    type Input = ImageCommand;
    type Output = ();

    async fn receive_input(
        &mut self,
        _context: &mut Context,
        command: Self::Input,
    ) -> KludgineResult<()> {
        match command {
            ImageCommand::SetSprite(sprite) => {
                self.sprite = sprite;
            }
            ImageCommand::SetTag(tag) => {
                self.sprite.set_current_tag(tag).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Component for Image {
    async fn update(&mut self, context: &mut SceneContext) -> KludgineResult<()> {
        self.current_frame = Some(
            self.sprite
                .get_frame(context.scene().elapsed().await)
                .await?,
        );
        Ok(())
    }

    async fn render(&self, context: &mut StyledContext, location: &Layout) -> KludgineResult<()> {
        if let Some(frame) = &self.current_frame {
            let render_bounds = location.inner_bounds();
            let target_bounds = match &self.options.scaling {
                None => Rect::sized(
                    render_bounds.origin,
                    frame.location().await.size.into(),
                ),
                Some(scaling) => match scaling {
                    ImageScaling::Fill => location.inner_bounds(),
                    _ => {
                        let frame_size: Size<f32> = frame.location().await.size.into();
                        let horizontal_scale = render_bounds.size.width / frame_size.width;
                        let horizontal_fit = Rect::sized(render_bounds.origin, frame_size * horizontal_scale);
                        let vertical_scale = render_bounds.size.height / frame_size.height;
                        let vertical_fit = Rect::sized(render_bounds.origin ,frame_size * vertical_scale);

                        match scaling {
                            ImageScaling::AspectFit => {
                                if render_bounds.approximately_contains_rect(&horizontal_fit) {
                                    horizontal_fit
                                } else {
                                    vertical_fit
                                }
                            },
                            
                            ImageScaling::AspectFill => {
                                if horizontal_fit.approximately_contains_rect(&render_bounds) {
                                    horizontal_fit
                                } else {
                                    vertical_fit
                                }
                            },
                            ImageScaling::Fill => unreachable!(),
                      }
                    }
                },
            };

            frame.render_within(context.scene(), target_bounds).await
        }
        Ok(())
    }

    async fn content_size(
        &self,
        _context: &mut StyledContext,
        _constraints: &Size<Option<f32>>,
    ) -> KludgineResult<Size> {
        if let Some(frame) = &self.current_frame {
            Ok(frame.location().await.size.into())
        } else {
            Ok(Size::default())
        }
    }
}

impl Image {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite,
            current_frame: None,
            options: ImageOptions::default(),
        }
    }

    pub fn options(mut self, options: ImageOptions) -> Self {
        self.options = options;
        self
    }
}
