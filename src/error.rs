use bevy::{ecs::query::QueryEntityError, prelude::*};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum FontError {
    #[error(
        "Unable to find font for {text}. A font has not been specified and DefaultFont has not been set."
    )]
    CannotFindFont { text: Entity },
    #[error("Entity {0} is not a FontCollection, {1}")]
    InvalidFont(Entity, QueryEntityError),
    #[error("Entity {0}, is not a ReactiveFont, {1}")]
    InvalidReactiveFont(Entity, QueryEntityError),
}
