use super::{
    runtime::Runtime,
    window::{RuntimeWindow, Window, WindowBuilder},
};
use async_trait::async_trait;
use std::marker::PhantomData;

#[async_trait]
pub trait Application: Sized + Send + Sync {
    // Async methods
    async fn initialize(&mut self) {}
    async fn should_exit(&mut self) -> bool {
        RuntimeWindow::count() == 0
    }
}

#[derive(Default)]
pub struct SingleWindowApplication<T> {
    phantom: PhantomData<T>,
}

pub trait WindowCreator<T> {
    fn get_window_builder() -> WindowBuilder {
        WindowBuilder::default().with_title(Self::window_title())
    }

    fn window_title() -> String {
        "Kludgine".to_owned()
    }
}

#[async_trait]
impl<T> Application for SingleWindowApplication<T> where
    T: Window + Default + WindowCreator<T> + 'static
{
}

impl<T> SingleWindowApplication<T>
where
    T: Window + Default + WindowCreator<T> + 'static,
{
    pub fn run(self) -> ! {
        Runtime::new(self).run(T::get_window_builder(), T::default())
    }
}