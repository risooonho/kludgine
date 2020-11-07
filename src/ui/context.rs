use async_handle::Handle;

use crate::{
    scene::Target,
    style::StyleSheet,
    ui::{
        Entity, EntityBuilder, HierarchicalArena, Index, Indexable, InteractiveComponent, Layout,
        UIState,
    },
};
mod layout_context;
mod styled_context;
pub use self::{
    layout_context::{LayoutContext, LayoutEngine},
    styled_context::StyledContext,
};
use super::{node::NodeData, LayerIndex};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Context {
    index: Index,
    arena: HierarchicalArena,
    ui_state: UIState,
    scene: Target,
}

impl Context {
    pub(crate) fn new<I: Indexable>(
        index: I,
        arena: HierarchicalArena,
        ui_state: UIState,
        scene: Target,
    ) -> Self {
        Self {
            index: index.index(),
            arena,
            ui_state,
            scene,
        }
    }

    pub async fn insert_new_entity<
        T: InteractiveComponent + 'static,
        I: Indexable,
        Message: Send + Sync + 'static,
    >(
        &self,
        parent: I,
        component: T,
    ) -> EntityBuilder<T, Message> {
        EntityBuilder::new(
            Some(parent.index()),
            component,
            &self.scene,
            Some(&self.layer_index().await.layer),
            &self.ui_state,
            &self.arena,
        )
    }

    pub fn index(&self) -> Index {
        self.index
    }

    pub async fn layer_index(&self) -> LayerIndex {
        self.ui_state
            .layer_for(self.index, &self.arena)
            .await
            .map(|layer| LayerIndex {
                index: self.index,
                layer,
            })
            .expect("can't use a node that is detatched from the ui hierarchy")
    }

    pub fn scene(&self) -> &'_ Target {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &'_ mut Target {
        &mut self.scene
    }

    pub fn entity<T: InteractiveComponent>(&self) -> Entity<T> {
        Entity::new(self.clone())
    }

    pub async fn set_parent<I: Indexable>(&self, parent: Option<I>) {
        self.arena.set_parent(self.index, parent).await
    }

    pub async fn add_child<I: Indexable>(&self, child: I) {
        self.arena.set_parent(child, Some(self.index)).await
    }

    pub async fn remove<I: Indexable>(&self, element: &I) {
        let index = element.index();
        self.arena.remove(&index).await;
        self.ui_state.removed_element(index).await;
    }

    pub async fn children(&self) -> Vec<Index> {
        self.arena.children(&Some(self.index)).await
    }

    pub fn clone_for<I: Indexable>(&self, index: &I) -> Self {
        Self {
            index: index.index(),
            arena: self.arena.clone(),
            ui_state: self.ui_state.clone(),
            scene: self.scene.clone(),
        }
    }

    pub async fn last_layout_for<I: Indexable>(&self, entity: I) -> Layout {
        let node = self.arena.get(&entity.index()).await.unwrap();
        node.last_layout().await
    }

    pub(crate) fn arena(&self) -> &'_ HierarchicalArena {
        &self.arena
    }

    pub fn new_layer<C: InteractiveComponent + 'static>(
        &self,
        layer_root: C,
    ) -> EntityBuilder<C, ()> {
        EntityBuilder::new(
            None,
            layer_root,
            &self.scene,
            None,
            &self.ui_state,
            &self.arena,
        )
    }

    pub async fn activate(&self) {
        self.layer_index()
            .await
            .layer
            .activate(self.index, &self.ui_state)
            .await;
    }

    pub async fn deactivate(&self) {
        self.layer_index()
            .await
            .layer
            .deactivate(&self.ui_state)
            .await;
    }

    pub async fn style_sheet(&self) -> StyleSheet {
        let node = self.arena.get(&self.index).await.unwrap();
        node.style_sheet().await
    }

    pub async fn focus(&self) {
        self.layer_index()
            .await
            .layer
            .focus_on(Some(self.index), &self.ui_state)
            .await;
    }

    pub async fn is_focused(&self) -> bool {
        self.layer_index()
            .await
            .layer
            .focus()
            .await
            .map(|focus| focus == self.index)
            .unwrap_or_default()
    }

    pub async fn blur(&self) {
        self.layer_index()
            .await
            .layer
            .focus_on(None, &self.ui_state)
            .await;
    }

    pub async fn set_style_sheet(&self, sheet: StyleSheet) {
        let node = self.arena.get(&self.index).await.unwrap();
        node.set_style_sheet(sheet).await
    }

    pub async fn set_needs_redraw(&self) {
        self.ui_state.set_needs_redraw().await;
    }

    pub async fn estimate_next_frame(&self, duration: Duration) {
        self.ui_state.estimate_next_frame(duration).await;
    }

    pub async fn estimate_next_frame_instant(&self, instant: Instant) {
        self.ui_state.estimate_next_frame_instant(instant).await;
    }

    pub async fn get_component_from<C: InteractiveComponent + 'static, T: Send + Sync + 'static>(
        &self,
        entity: Entity<C>,
    ) -> Option<Handle<T>> {
        let node = self.arena.get(&entity).await.unwrap();
        let component = node.component.read().await;
        let node = component.as_any().downcast_ref::<NodeData<C>>()?;

        node.component::<T>().await
    }
}
