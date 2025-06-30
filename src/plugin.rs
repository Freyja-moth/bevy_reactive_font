use crate::prelude::*;
use bevy::{ecs::relationship::Relationship, prelude::*};

/// Updates the font for the entity it is triggered on.
#[derive(Event)]
pub struct UpdateFont;

/// Updates the [`FontSize`] for the entity it is triggered on.
#[derive(Event)]
pub struct UpdateFontSize;

/// Updates the [`FontColor`] for the entity it is triggered on.
#[derive(Event)]
pub struct UpdateFontColor;

/// A plugin that manages [`ReactiveFont`]'s and [`FontCollection`]'s
pub struct ReactiveFontPlugin;

impl Plugin for ReactiveFontPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_reactive_font)
            .add_systems(
                Update,
                (
                    default_font_changed.run_if(resource_changed::<DefaultFont>),
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

fn on_add_reactive_font(on_add: Trigger<OnAdd, ReactiveFont>, mut commands: Commands) {
    commands
        .entity(on_add.target())
        .trigger(UpdateFont)
        .trigger(UpdateFontSize)
        .trigger(UpdateFontColor);
}

fn default_font_changed(
    mut commands: Commands,
    fonts: Populated<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
) {
    let entities = fonts.iter().collect::<Vec<_>>();

    commands.trigger_targets(UpdateFont, entities);
}

#[allow(clippy::type_complexity)]
fn font_handle_changed(
    mut commands: Commands,
    default_font: Res<DefaultFont>,
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
    if font_handles.contains(default_font.0) {
        let entities = fonts.iter().collect::<Vec<_>>();

        if !entities.is_empty() {
            commands.trigger_targets(UpdateFont, entities);
        }
    }

    let entities = font_handles
        .iter()
        .flat_map(|used_by| used_by.iter())
        .collect::<Vec<_>>();

    if !entities.is_empty() {
        commands.trigger_targets(UpdateFont, entities);
    }
}

fn default_font_size_changed(
    mut commands: Commands,
    default_font: Res<DefaultFont>,
    fonts: Query<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
    font_handles: Populated<&UsedBy, Changed<DefaultFontSize>>,
) {
    // If the default font has changed, update all fonts that are using it
    if font_handles.contains(default_font.0) {
        let entities = fonts.iter().collect::<Vec<_>>();

        if !entities.is_empty() {
            commands.trigger_targets(UpdateFontSize, entities);
        }
    }

    let entities = font_handles
        .iter()
        .flat_map(|used_by| used_by.iter())
        .collect::<Vec<_>>();

    if !entities.is_empty() {
        commands.trigger_targets(UpdateFontSize, entities);
    }
}

fn default_font_color_changed(
    mut commands: Commands,
    default_font: Res<DefaultFont>,
    fonts: Query<Entity, (With<ReactiveFont>, Without<UsingFont>)>,
    font_handles: Populated<&UsedBy, Changed<DefaultFontColor>>,
) {
    // If the default font has changed, update all fonts that are using it
    if font_handles.contains(default_font.0) {
        let entities = fonts.iter().collect::<Vec<_>>();

        commands.trigger_targets(UpdateFontColor, entities);
    }

    let entities = font_handles
        .iter()
        .flat_map(|used_by| used_by.iter())
        .collect::<Vec<_>>();

    if !entities.is_empty() {
        commands.trigger_targets(UpdateFontColor, entities);
    }
}

// Font Handles

fn selected_font(on_add: Trigger<OnAdd, UsingFont>, mut commands: Commands) {
    commands.entity(on_add.target()).trigger(UpdateFont);
}

fn deselected_font(on_remove: Trigger<OnRemove, UsingFont>, mut commands: Commands) {
    commands.entity(on_remove.target()).trigger(UpdateFont);
}

fn on_add_font_tag(on_add: Trigger<OnAdd, (Bold, Italic)>, mut commands: Commands) {
    commands.entity(on_add.target()).trigger(UpdateFont);
}

fn on_remove_font_tag(on_remove: Trigger<OnRemove, (Bold, Italic)>, mut commands: Commands) {
    commands.entity(on_remove.target()).trigger(UpdateFont);
}

#[allow(clippy::type_complexity)]
fn update_font(
    update: Trigger<UpdateFont>,
    mut reactive_fonts: Populated<(&mut TextFont, Has<Italic>, Has<Bold>, Option<&UsingFont>)>,
    fonts: Populated<(&RegularFont, &ItalicFont, &BoldFont, &BoldItalicFont), With<FontCollection>>,
    default_font: Option<Res<DefaultFont>>,
) -> Result<(), BevyError> {
    let (mut text_font, is_italic, is_bold, using_font) =
        reactive_fonts.get_mut(update.target())?;

    let current_font = using_font
        .map(UsingFont::get)
        .or(default_font.map(|font| font.0))
        .ok_or(FontError::CannotFindFont {
            text: update.target(),
        })?;

    let (
        RegularFont(regular_font),
        ItalicFont(italic_font),
        BoldFont(bold_font),
        BoldItalicFont(bold_italic_font),
    ) = fonts.get(current_font)?;

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

fn on_add_font_size(on_add: Trigger<OnAdd, FontSize>, mut commands: Commands) {
    commands.entity(on_add.target()).trigger(UpdateFontSize);
}

fn changed_font_size(
    mut commands: Commands,
    changed: Populated<Entity, (With<ReactiveFont>, Changed<FontSize>)>,
) {
    let entities = changed.iter().collect::<Vec<_>>();

    commands.trigger_targets(UpdateFontSize, entities);
}

fn on_remove_font_size(on_remove: Trigger<OnRemove, FontSize>, mut commands: Commands) {
    commands.entity(on_remove.target()).trigger(UpdateFontSize);
}

fn update_font_size(
    update: Trigger<UpdateFontSize>,
    mut reactive_fonts: Query<(&mut TextFont, Option<&FontSize>, Option<&UsingFont>)>,
    fonts: Query<&DefaultFontSize, With<FontCollection>>,
    default_font: Option<Res<DefaultFont>>,
) -> Result<(), BevyError> {
    let (mut text_font, font_size, using_font) = reactive_fonts.get_mut(update.target())?;

    let current_font = using_font
        .map(UsingFont::get)
        .or(default_font.map(|font| font.0))
        .ok_or(FontError::CannotFindFont {
            text: update.target(),
        })?;

    let default_font_size = fonts.get(current_font)?;

    text_font.font_size = font_size
        .map(FontSize::into_inner)
        .unwrap_or(default_font_size.0);

    Ok(())
}

// Font Color

fn on_add_font_color(on_add: Trigger<OnAdd, FontColor>, mut commands: Commands) {
    commands.entity(on_add.target()).trigger(UpdateFontColor);
}

fn changed_font_color(
    mut commands: Commands,
    changed: Populated<Entity, (With<ReactiveFont>, Changed<FontColor>)>,
) {
    let entities = changed.iter().collect::<Vec<_>>();

    commands.trigger_targets(UpdateFontColor, entities);
}

fn on_remove_font_color(on_remove: Trigger<OnRemove, FontColor>, mut commands: Commands) {
    commands.entity(on_remove.target()).trigger(UpdateFontColor);
}

fn update_font_color(
    update: Trigger<UpdateFontColor>,
    mut reactive_fonts: Query<(&mut TextColor, Option<&FontColor>, Option<&UsingFont>)>,
    fonts: Query<&DefaultFontColor, With<FontCollection>>,
    default_font: Option<Res<DefaultFont>>,
) -> Result<(), BevyError> {
    let (mut text_color, font_color, using_font) = reactive_fonts.get_mut(update.target())?;

    let current_font = using_font
        .map(UsingFont::get)
        .or(default_font.map(|font| font.0))
        .ok_or(FontError::CannotFindFont {
            text: update.target(),
        })?;

    let default_font_color = fonts.get(current_font)?;

    text_color.0 = font_color
        .map(FontColor::into_inner)
        .unwrap_or(default_font_color.0);

    Ok(())
}
