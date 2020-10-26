extern crate kludgine;
use kludgine::prelude::*;

fn main() {
    SingleWindowApplication::run(UIExample::default());
}

#[derive(Default)]
struct UIExample {
    image: Entity<Image>,
    label: Entity<Label>,
    button: Entity<Button>,
    text_field: Entity<TextField>,
    new_window_button: Entity<Button>,
    current_count: usize,
}

impl WindowCreator for UIExample {
    fn window_title() -> String {
        "User Interface - Kludgine".to_owned()
    }

    fn initial_system_theme() -> SystemTheme {
        SystemTheme::Dark
    }
}

impl Window for UIExample {}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonClicked,
    NewWindowClicked,
    LabelClicked,
    TextFieldEvent(TextFieldEvent),
}

#[async_trait]
impl InteractiveComponent for UIExample {
    type Message = Message;
    type Command = ();
    type Event = ();

    async fn receive_message(
        &mut self,
        _context: &mut Context,
        message: Self::Message,
    ) -> KludgineResult<()> {
        match message {
            Message::LabelClicked => {
                self.current_count += 0;
                self.label
                    .send(LabelCommand::SetValue("You clicked me".to_string()))
                    .await?;
            }
            Message::ButtonClicked => {
                self.current_count += 1;
                self.label
                    .send(LabelCommand::SetValue(self.current_count.to_string()))
                    .await?;
            }
            Message::NewWindowClicked => {
                Runtime::open_window(Self::get_window_builder(), UIExample::default()).await;
            }
            Message::TextFieldEvent(event) => {
                if let TextFieldEvent::ValueChanged(text) = event {
                    self.label
                        .send(LabelCommand::SetValue(text.to_string().await))
                        .await?;
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Component for UIExample {
    async fn initialize(&mut self, context: &mut Context) -> KludgineResult<()> {
        let sprite = include_aseprite_sprite!("assets/stickguy").await?;
        self.image = self
            .new_entity(context, Image::new(sprite))
            .style_sheet(Style::new().with(BackgroundColor(Color::new(0.0, 1.0, 1.0, 1.0).into())))
            .bounds(AbsoluteBounds {
                right: Dimension::from_f32(10.),
                bottom: Dimension::from_f32(10.),
                ..Default::default()
            })
            .insert()
            .await?;

        self.text_field = self
            .new_entity(
                context,
                TextField::new(RichText::new(vec![Text::span(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt",
                    Default::default(),
                )])),
            )
            .bounds(AbsoluteBounds {
                left: Dimension::from_f32(32.),
                right: Dimension::from_f32(32.),
                top: Dimension::from_f32(32.),
                ..Default::default()
            })
            .callback(Message::TextFieldEvent)
            .insert()
            .await?;

        self.label = self
            .new_entity(context, Label::new("Test Label"))
            .style_sheet(
                Style::new()
                    .with(ForegroundColor(Color::new(1.0, 1.0, 1.0, 0.1).into()))
                    .with(LabelBackgroundColor(Color::new(1.0, 0.0, 1.0, 0.5).into()))
                    .with(FontSize::new(72.))
                    .with(Alignment::Right),
            )
            .bounds(AbsoluteBounds {
                left: Dimension::from_f32(32.),
                right: Dimension::from_f32(32.),
                top: Dimension::from_f32(96.),
                bottom: Dimension::from_f32(64.),
                ..Default::default()
            })
            .callback(|_| Message::LabelClicked)
            .insert()
            .await?;

        self.button = self
            .new_entity(context, Button::new("Press Me"))
            .normal_style(Style::new().with(BackgroundColor(Color::ROYALBLUE.into())))
            .bounds(AbsoluteBounds {
                bottom: Dimension::from_f32(10.),

                ..Default::default()
            })
            .callback(|_| Message::ButtonClicked)
            .insert()
            .await?;

        self.new_window_button = self
            .new_entity(context, Button::new("New Window"))
            .bounds(AbsoluteBounds {
                bottom: Dimension::from_f32(10.),
                left: Dimension::from_f32(10.),
                ..Default::default()
            })
            .callback(|_| Message::NewWindowClicked)
            .insert()
            .await?;

        Ok(())
    }
}
