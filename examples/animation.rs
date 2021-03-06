extern crate kludgine;
use kludgine::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    SingleWindowApplication::run(Animation::default());
}

#[derive(Default)]
struct Animation {
    image: Entity<Image>,
    manager: RequiresInitialization<AnimationManager<ImageAlphaAnimation>>,
    frame_manager: RequiresInitialization<AnimationManager<ImageFrameAnimation>>,
    fade_in: bool,
}

impl WindowCreator for Animation {
    fn window_title() -> String {
        "Animation - Kludgine".to_owned()
    }
}

impl Window for Animation {}

impl StandaloneComponent for Animation {}

#[async_trait]
impl Component for Animation {
    async fn initialize(&mut self, context: &mut Context) -> KludgineResult<()> {
        context
            .set_style_sheet(
                Style::new()
                    .with(BackgroundColor(Color::GREEN.into()))
                    .into(),
            )
            .await;
        let sprite = include_aseprite_sprite!("assets/stickguy").await?;
        sprite.set_current_tag(Some("Idle")).await?;
        self.image = self
            .new_entity(context, Image::new(sprite))
            .await?
            .bounds(
                AbsoluteBounds::default()
                    .with_left(Points::new(30.))
                    .with_top(Points::new(30.)),
            )
            .insert()
            .await?;

        self.manager.initialize_with(
            AnimationManager::new(self.image.animate().alpha(0.3, LinearTransition)).await,
        );

        self.frame_manager.initialize_with(
            AnimationManager::new(self.image.animate().tagged_frame(
                "WalkRight",
                0.0,
                LinearTransition,
            ))
            .await,
        );

        self.fade().await;

        Ok(())
    }

    async fn update(&mut self, context: &mut Context) -> KludgineResult<()> {
        self.manager.update(context).await;
        self.frame_manager.update(context).await;
        Ok(())
    }

    async fn clicked(
        &mut self,
        _context: &mut Context,
        _window_position: Point<f32, Scaled>,
        _button: MouseButton,
    ) -> KludgineResult<()> {
        self.fade().await;
        Ok(())
    }
}

impl Animation {
    async fn fade(&mut self) {
        self.fade_in = !self.fade_in;
        let target_opacity = if self.fade_in { 1.0 } else { 0.1 };
        let now = Instant::now();
        let completion_time = now.checked_add(Duration::from_secs(1)).unwrap();
        self.manager.push_frame(
            self.image.animate().alpha(target_opacity, LinearTransition),
            completion_time,
        );

        let direction = if self.fade_in {
            "WalkLeft"
        } else {
            "WalkRight"
        };

        self.frame_manager.push_frame(
            self.image
                .animate()
                .tagged_frame(direction, 0.0, LinearTransition),
            now,
        );
        self.frame_manager.push_frame(
            self.image
                .animate()
                .tagged_frame(direction, 1.0, LinearTransition),
            completion_time,
        );
    }
}
