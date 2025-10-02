use crate::prelude::*;
use bevy::{
    ecs::{query::QueryEntityError, relationship::Relationship},
    prelude::*,
};

/// Updates the font for the entity it is triggered on.
#[derive(EntityEvent)]
pub struct UpdateFont(Entity);

/// Updates the [`FontSize`] for the entity it is triggered on.
#[derive(EntityEvent)]
pub struct UpdateFontSize(Entity);

/// Updates the [`FontColor`] for the entity it is triggered on.
#[derive(EntityEvent)]
pub struct UpdateFontColor(Entity);

/// A plugin that manages [`ReactiveFont`]'s and [`FontCollection`]'s
pub struct ReactiveFontPlugin;

impl Plugin for ReactiveFontPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_reactive_font)
            .add_systems(
                Update,
                (
                    default_font_changed.run_if(resource_exists_and_changed::<DefaultFont>),
                    font_handle_changed,
                    default_font_size_changed,
                    default_font_color_changed,
                ),
            )
            // Font Handles
            .add_observer(selected_font)
            .add_observer(deselected_font)
            .add_observer(on_add_font_tag)
            .add_observer(on_remove_font_tag)
            .add_observer(update_font)
            // Font Size
            .add_observer(on_add_font_size)
            .add_systems(Update, changed_font_size)
            .add_observer(on_remove_font_size)
            .add_observer(update_font_size)
            // Font Color
            .add_observer(on_add_font_color)
            .add_systems(Update, changed_font_color)
            .add_observer(on_remove_font_color)
            .add_observer(update_font_color);
    }
}

fn on_add_reactive_font(on_add: On<Add, ReactiveFont>, mut commands: Commands) {
    commands
        .entity(on_add.entity)
        .trigger(UpdateFont)
        .trigger(UpdateFontSize)
        .trigger(UpdateFontColor);
}

fn default_font_changed(
    mut commands: Commands,
    fonts: Populated<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
) {
    fonts.iter().for_each(|entity| {
        commands.entity(entity).trigger(UpdateFont);
    });
}

#[allow(clippy::type_complexity)]
fn font_handle_changed(
    mut commands: Commands,
    default_font: Option<Res<DefaultFont>>,
    fonts: Query<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
    font_handles: Populated<
        &UsedBy,
        Or<(
            Changed<RegularFont>,
            Changed<BoldFont>,
            Changed<ItalicFont>,
            Changed<BoldItalicFont>,
        )>,
    >,
) {
    // If the default font has changed, update all fonts that are using it
    if default_font.is_some_and(|default_font| font_handles.contains(default_font.0)) {
        fonts.iter().for_each(|entity| {
            commands.entity(entity).trigger(UpdateFont);
        })
    }

    font_handles
        .iter()
        .flat_map(|used_by| used_by.iter())
        .for_each(|entity| {
            commands.entity(entity).trigger(UpdateFont);
        });
}

fn default_font_size_changed(
    mut commands: Commands,
    default_font: Option<Res<DefaultFont>>,
    fonts: Query<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
    font_handles: Populated<&UsedBy, Changed<DefaultFontSize>>,
) {
    // If the default font has changed, update all fonts that are using it
    if default_font.is_some_and(|default_font| font_handles.contains(default_font.0)) {
        fonts.iter().for_each(|entity| {
            commands.entity(entity).trigger(UpdateFontSize);
        });
    }

    font_handles
        .iter()
        .flat_map(|used_by| used_by.iter())
        .for_each(|entity| {
            commands.entity(entity).trigger(UpdateFontSize);
        });
}

fn default_font_color_changed(
    mut commands: Commands,
    default_font: Option<Res<DefaultFont>>,
    fonts: Query<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
    font_handles: Populated<&UsedBy, Changed<DefaultFontColor>>,
) {
    // If the default font has changed, update all fonts that are using it
    if default_font.is_some_and(|default_font| font_handles.contains(default_font.0)) {
        fonts.iter().for_each(|entity| {
            commands.entity(entity).trigger(UpdateFontColor);
        });
    }

    font_handles
        .iter()
        .flat_map(|used_by| used_by.iter())
        .for_each(|entity| {
            commands.entity(entity).trigger(UpdateFontColor);
        });
}

// Font Handles

fn selected_font(on_add: On<Add, UsingFont>, mut commands: Commands) {
    commands.entity(on_add.entity).trigger(UpdateFont);
}

fn deselected_font(on_remove: On<Remove, UsingFont>, mut commands: Commands) {
    commands.entity(on_remove.entity).trigger(UpdateFont);
}

fn on_add_font_tag(on_add: On<Add, (Bold, Italic)>, mut commands: Commands) {
    commands.entity(on_add.entity).trigger(UpdateFont);
}

fn on_remove_font_tag(on_remove: On<Remove, (Bold, Italic)>, mut commands: Commands) {
    commands.entity(on_remove.entity).trigger(UpdateFont);
}

#[allow(clippy::type_complexity)]
fn update_font(
    update: On<UpdateFont>,
    mut reactive_fonts: Populated<(&mut TextFont, Has<Italic>, Has<Bold>, Option<&UsingFont>)>,
    fonts: Populated<(&RegularFont, &ItalicFont, &BoldFont, &BoldItalicFont), With<FontCollection>>,
    default_font: Option<Res<DefaultFont>>,
) -> Result<(), BevyError> {
    if let Err(QueryEntityError::EntityDoesNotExist(_)) = reactive_fonts.get_mut(update.0) {
        // Happens when the entity has been despawned, ignore it.
        return Ok(());
    }

    let (mut text_font, is_italic, is_bold, using_font) = reactive_fonts
        .get_mut(update.0)
        .map_err(|err| FontError::InvalidReactiveFont(update.0, err))?;

    let current_font = using_font
        .map(UsingFont::get)
        .or(default_font.map(|font| font.0))
        .ok_or(FontError::CannotFindFont { text: update.0 })?;

    let (
        RegularFont(regular_font),
        ItalicFont(italic_font),
        BoldFont(bold_font),
        BoldItalicFont(bold_italic_font),
    ) = fonts
        .get(current_font)
        .map_err(|err| FontError::InvalidFont(update.0, err))?;

    let font = match (is_italic, is_bold) {
        (true, true) => bold_italic_font,
        (true, _) => italic_font,
        (_, true) => bold_font,
        _ => regular_font,
    };

    text_font.font = font.clone();

    Ok(())
}

// Font Size

fn on_add_font_size(on_add: On<Add, FontSize>, mut commands: Commands) {
    commands.entity(on_add.entity).trigger(UpdateFontSize);
}

fn changed_font_size(
    mut commands: Commands,
    changed: Populated<Entity, (With<ReactiveFont>, Changed<FontSize>)>,
) {
    changed.iter().for_each(|entity| {
        commands.entity(entity).trigger(UpdateFontSize);
    });
}

fn on_remove_font_size(on_remove: On<Remove, FontSize>, mut commands: Commands) {
    commands.entity(on_remove.entity).trigger(UpdateFontSize);
}

fn update_font_size(
    update: On<UpdateFontSize>,
    mut reactive_fonts: Query<(&mut TextFont, Option<&FontSize>, Option<&UsingFont>)>,
    fonts: Query<&DefaultFontSize, With<FontCollection>>,
    default_font: Option<Res<DefaultFont>>,
) -> Result<(), BevyError> {
    if let Err(QueryEntityError::EntityDoesNotExist(_)) = reactive_fonts.get_mut(update.0) {
        // Happens when the entity has been despawned, ignore it.
        return Ok(());
    }

    let (mut text_font, font_size, using_font) = reactive_fonts
        .get_mut(update.0)
        .map_err(|err| FontError::InvalidReactiveFont(update.0, err))?;

    let current_font = using_font
        .map(UsingFont::get)
        .or(default_font.map(|font| font.0))
        .ok_or(FontError::CannotFindFont { text: update.0 })?;

    let default_font_size = fonts
        .get(current_font)
        .map_err(|err| FontError::InvalidFont(update.0, err))?;

    text_font.font_size = font_size
        .map(FontSize::into_inner)
        .unwrap_or(default_font_size.0);

    Ok(())
}

// Font Color

fn on_add_font_color(on_add: On<Add, FontColor>, mut commands: Commands) {
    commands.entity(on_add.entity).trigger(UpdateFontColor);
}

fn changed_font_color(
    mut commands: Commands,
    changed: Populated<Entity, (With<ReactiveFont>, Changed<FontColor>)>,
) {
    changed.iter().for_each(|entity| {
        commands.entity(entity).trigger(UpdateFontColor);
    });
}

fn on_remove_font_color(on_remove: On<Remove, FontColor>, mut commands: Commands) {
    commands.entity(on_remove.entity).trigger(UpdateFontColor);
}

fn update_font_color(
    update: On<UpdateFontColor>,
    mut reactive_fonts: Query<(&mut TextColor, Option<&FontColor>, Option<&UsingFont>)>,
    fonts: Query<&DefaultFontColor, With<FontCollection>>,
    default_font: Option<Res<DefaultFont>>,
) -> Result<(), BevyError> {
    if let Err(QueryEntityError::EntityDoesNotExist(_)) = reactive_fonts.get_mut(update.0) {
        // Happens when the entity has been despawned, ignore it.
        return Ok(());
    }

    let (mut text_color, font_color, using_font) = reactive_fonts
        .get_mut(update.0)
        .map_err(|err| FontError::InvalidReactiveFont(update.0, err))?;

    let current_font = using_font
        .map(UsingFont::get)
        .or(default_font.map(|font| font.0))
        .ok_or(FontError::CannotFindFont { text: update.0 })?;

    let default_font_color = fonts
        .get(current_font)
        .map_err(|err| FontError::InvalidFont(update.0, err))?;

    text_color.0 = font_color
        .map(FontColor::into_inner)
        .unwrap_or(default_font_color.0);

    Ok(())
}
