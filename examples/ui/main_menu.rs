use kludgine::prelude::*;

#[derive(Debug)]
pub struct MainMenu;

#[derive(Clone, Debug)]
pub enum MainMenuMessage {
    ButtonEvent(ScrollEvent<GridEvent<MainMenuOptions, ControlEvent>>),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum MainMenuOptions {
    Old,
}

#[async_trait]
impl Component for MainMenu {
    async fn initialize(&mut self, context: &mut Context) -> KludgineResult<()> {
        self.new_entity(
            context,
            Scroll::new(
                Grid::rows()
                    .cell(
                        MainMenuOptions::Old,
                        Button::new("Open Old Testbed"),
                        Dimension::Auto,
                    )
                    .build(),
            ),
        )
        .await?
        .callback(&self.entity(context), MainMenuMessage::ButtonEvent)
        .insert()
        .await?;
        Ok(())
    }
}

#[async_trait]
impl InteractiveComponent for MainMenu {
    type Message = MainMenuMessage;
    type Command = ();
    type Event = ();

    async fn receive_message(
        &mut self,
        _context: &mut Context,
        message: Self::Message,
    ) -> KludgineResult<()> {
        let MainMenuMessage::ButtonEvent(event) = message;
        let ScrollEvent::Child(event) = event;
        match event.key {
            MainMenuOptions::Old => {
                Runtime::open_window(
                    crate::old::UIExample::get_window_builder(),
                    crate::old::UIExample::default(),
                )
                .await
            }
        }
        Ok(())
    }
}

impl WindowCreator for MainMenu {
    fn window_title() -> String {
        "User Interface - Kludgine".to_owned()
    }

    fn initial_system_theme() -> SystemTheme {
        SystemTheme::Dark
    }
}

impl Window for MainMenu {}
