use bevy::prelude::*;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum FontError {
    #[error(
        "Unable to find font for {text}. A font has not been specified and DefaultFont has not been set."
    )]
    CannotFindFont { text: Entity },
}
