use bevy::{
    ecs::{
        entity::MapEntities,
        relationship::{OrderedRelationshipSourceCollection, RelationshipSourceCollection},
    },
    prelude::*,
};

/// A relationship source that reports that it's never empty.
///
/// The only relationship implementation removes [`RelationshipTarget`]'s when they are
/// empty, which is beyond frustrating. This has been marked as *intended* and won't be fixed.
#[derive(Reflect, DerefMut, Deref, PartialEq, Eq, Clone, Debug)]
pub struct NeverEmptyVec<T>(Vec<T>);

impl<T> Default for NeverEmptyVec<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl RelationshipSourceCollection for NeverEmptyVec<Entity> {
    type SourceIter<'a> = core::iter::Copied<core::slice::Iter<'a, Entity>>;

    fn new() -> Self {
        Self::default()
    }

    fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add(&mut self, entity: Entity) -> bool {
        self.0.push(entity);

        true
    }

    fn remove(&mut self, entity: Entity) -> bool {
        if let Some(index) = self.0.iter().position(|e| e == entity) {
            self.0.remove(index);
            return true;
        }

        false
    }

    fn iter(&self) -> Self::SourceIter<'_> {
        <[Entity]>::iter(&self.0).copied()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.0.extend(entities);
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl OrderedRelationshipSourceCollection for NeverEmptyVec<Entity> {
    fn insert(&mut self, index: usize, entity: Entity) {
        self.0.push(entity);
        let len = self.len();
        if index < len {
            self.0.swap(index, len - 1);
        }
    }

    fn remove_at(&mut self, index: usize) -> Option<Entity> {
        (index < self.len()).then(|| self.0.swap_remove(index))
    }

    fn insert_stable(&mut self, index: usize, entity: Entity) {
        if index < self.len() {
            self.0.insert(index, entity);
        } else {
            self.0.push(entity);
        }
    }

    fn remove_at_stable(&mut self, index: usize) -> Option<Entity> {
        (index < self.len()).then(|| self.0.remove(index))
    }

    fn sort(&mut self) {
        self.0.sort_unstable();
    }

    fn insert_sorted(&mut self, entity: Entity) {
        let index = self.0.partition_point(|e| e <= &entity);
        self.insert_stable(index, entity);
    }

    fn place_most_recent(&mut self, index: usize) {
        if let Some(entity) = self.0.pop() {
            let index = index.min(self.0.len());
            self.insert(index, entity);
        }
    }

    fn place(&mut self, entity: Entity, index: usize) {
        if let Some(current) = <[Entity]>::iter(&self.0).position(|e| *e == entity) {
            let index = index.min(self.len());
            self.0.remove(current);
            self.insert(index, entity);
        };
    }
}

impl MapEntities for NeverEmptyVec<Entity> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for entities in self.0.iter_mut() {
            entities.map_entities(entity_mapper);
        }
    }
}
