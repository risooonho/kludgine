use crate::{
    math::{Scaled, Size},
    runtime::Runtime,
    style::theme::{Minimal, SystemTheme, Theme},
    ui::InteractiveComponent,
    Handle, KludgineError, KludgineResult,
};
use async_trait::async_trait;
use crossbeam::sync::ShardedLock;
use easygpu::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use winit::window::{WindowBuilder as WinitWindowBuilder, WindowId};

pub mod event;
pub(crate) mod frame;
mod renderer;
mod runtime_window;

pub(crate) use runtime_window::RuntimeWindow;

pub use winit::window::Icon;

/// How to react to a request to close a window
pub enum CloseResponse {
    /// Window should remain open
    RemainOpen,
    /// Window should close
    Close,
}

/// Trait to implement a Window
#[async_trait]
pub trait Window: InteractiveComponent + Send + Sync + 'static {
    /// The window was requested to be closed, most likely from the Close Button. Override
    /// this implementation if you want logic in place to prevent a window from closing.
    async fn close_requested(&self) -> KludgineResult<CloseResponse> {
        Ok(CloseResponse::Close)
    }

    /// Specify a target frames per second, which will force your window
    /// to redraw at this rate. If None is returned, the Window will only
    /// redraw when requested via methods on Context.
    fn target_fps(&self) -> Option<u16> {
        None
    }

    fn theme(&self) -> Theme {
        Minimal::default().theme()
    }
}

pub trait WindowCreator: Window {
    fn get_window_builder() -> WindowBuilder {
        WindowBuilder::default()
            .with_title(Self::window_title())
            .with_initial_system_theme(Self::initial_system_theme())
            .with_size(Self::initial_size())
            .with_resizable(Self::resizable())
            .with_maximized(Self::maximized())
            .with_visible(Self::visible())
            .with_transparent(Self::transparent())
            .with_decorations(Self::decorations())
            .with_always_on_top(Self::always_on_top())
    }

    fn window_title() -> String {
        "Kludgine".to_owned()
    }

    fn initial_size() -> Size<u32, Scaled> {
        Size::new(1024, 768)
    }

    fn resizable() -> bool {
        true
    }

    fn maximized() -> bool {
        false
    }

    fn visible() -> bool {
        true
    }

    fn transparent() -> bool {
        false
    }

    fn decorations() -> bool {
        true
    }

    fn always_on_top() -> bool {
        false
    }

    fn initial_system_theme() -> SystemTheme {
        SystemTheme::Light
    }
}

#[derive(Default)]
pub struct WindowBuilder {
    title: Option<String>,
    size: Option<Size<u32, Scaled>>,
    resizable: Option<bool>,
    maximized: Option<bool>,
    visible: Option<bool>,
    transparent: Option<bool>,
    decorations: Option<bool>,
    always_on_top: Option<bool>,
    pub(crate) initial_system_theme: Option<SystemTheme>,
    icon: Option<winit::window::Icon>,
}

impl WindowBuilder {
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_size(mut self, size: Size<u32, Scaled>) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = Some(maximized);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = Some(transparent);
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = Some(decorations);
        self
    }

    pub fn with_always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = Some(always_on_top);
        self
    }

    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_initial_system_theme(mut self, system_theme: SystemTheme) -> Self {
        self.initial_system_theme = Some(system_theme);
        self
    }
}

impl Into<WinitWindowBuilder> for WindowBuilder {
    fn into(self) -> WinitWindowBuilder {
        let mut builder = WinitWindowBuilder::new();
        if let Some(title) = self.title {
            builder = builder.with_title(title);
        }
        if let Some(size) = self.size {
            builder =
                builder.with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
                    width: size.width,
                    height: size.height,
                }));
        }
        if let Some(resizable) = self.resizable {
            builder = builder.with_resizable(resizable);
        }
        if let Some(maximized) = self.maximized {
            builder = builder.with_maximized(maximized);
        }
        if let Some(visible) = self.visible {
            builder = builder.with_visible(visible);
        }
        if let Some(transparent) = self.transparent {
            builder = builder.with_transparent(transparent);
        }
        if let Some(decorations) = self.decorations {
            builder = builder.with_decorations(decorations);
        }
        if let Some(always_on_top) = self.always_on_top {
            builder = builder.with_always_on_top(always_on_top);
        }

        builder = builder.with_window_icon(self.icon);

        builder
    }
}

#[async_trait]
pub trait OpenableWindow {
    async fn open(window: Self);
}

#[async_trait]
impl<T> OpenableWindow for T
where
    T: Window + WindowCreator,
{
    async fn open(window: Self) {
        Runtime::open_window(Self::get_window_builder(), window).await
    }
}

lazy_static! {
    static ref WINDOW_CHANNELS: Handle<HashMap<WindowId, async_channel::Sender<WindowMessage>>> =
        Handle::new(HashMap::new());
}

lazy_static! {
    static ref WINDOWS: ShardedLock<HashMap<WindowId, RuntimeWindow>> =
        ShardedLock::new(HashMap::new());
}

pub(crate) enum WindowMessage {
    Close,
}

impl WindowMessage {
    pub async fn send_to(self, id: WindowId) -> KludgineResult<()> {
        let sender = {
            let mut channels = WINDOW_CHANNELS.write().await;
            if let Some(sender) = channels.get_mut(&id) {
                sender.clone()
            } else {
                return Err(KludgineError::InternalWindowMessageSendError(
                    "Channel not found for id".to_owned(),
                ));
            }
        };

        sender.send(self).await.unwrap_or_default();
        Ok(())
    }
}
