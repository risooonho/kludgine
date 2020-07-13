use crate::ui::Node;
use generational_arena::Arena;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

pub use generational_arena::Index;

pub(crate) struct HierarchicalArena {
    arena: Arena<Node>,
    children_by_parent: HashMap<Option<Index>, HashSet<Index>>,
    parents: HashMap<Index, Option<Index>>,
}

impl HierarchicalArena {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            children_by_parent: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    pub fn insert(&mut self, parent: Option<Index>, node: Node) -> Index {
        let index = self.arena.insert(node);

        self.set_parent(index, parent);

        index
    }

    pub fn set_parent(&mut self, child: Index, parent: Option<Index>) {
        if let Some(old_parent) = self.parents.get(&child) {
            self.children_by_parent
                .get_mut(old_parent)
                .unwrap()
                .remove(&child);
        }

        if let Some(new_parent) = parent {
            self.children_by_parent
                .entry(Some(new_parent))
                .and_modify(|children| {
                    children.insert(child);
                })
                .or_insert_with(|| hash_set!(child));
        }
    }

    pub fn children(&self, parent: &Option<Index>) -> HashSet<Index> {
        if let Some(children) = self.children_by_parent.get(parent) {
            children.clone()
        } else {
            HashSet::default()
        }
    }

    pub fn get(&self, index: Index) -> Option<&'_ Node> {
        self.arena.get(index)
    }

    pub fn get_mut(&mut self, index: Index) -> Option<&'_ mut Node> {
        self.arena.get_mut(index)
    }

    pub fn iter(&self) -> ArenaIterator<'_> {
        let queue = self.children(&None).into_iter().collect();
        ArenaIterator {
            arena: self,
            queue,
            processed: HashSet::new(),
        }
    }
}

pub struct ArenaIterator<'a> {
    arena: &'a HierarchicalArena,
    queue: VecDeque<Index>,
    processed: HashSet<Index>,
}

impl<'a> Iterator for ArenaIterator<'a> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.queue.pop_front() {
            if self.processed.contains(&index) {
                panic!("Cycle detected in hierarchy");
            }

            self.processed.insert(index);
            self.queue.extend(self.arena.children(&Some(index)));
            Some(index)
        } else {
            None
        }
    }
}
