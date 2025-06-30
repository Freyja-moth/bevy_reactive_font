use bevy::prelude::*;

/// Marks that a peice of text should be italic
#[derive(Component)]
pub struct Italic;

/// Marks that a peice of text should be bold
#[derive(Component)]
pub struct Bold;

/// How large the text should be.
///
/// This will ignore [`DefaultFontSize`] and will stay the same size even if [`DefaultFontSize`]
/// changes.
#[derive(Component, Deref)]
pub struct FontSize(pub f32);
impl Default for FontSize {
    fn default() -> Self {
        Self(15.)
    }
}
impl FontSize {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    pub fn into_inner(&self) -> f32 {
        self.0
    }
}

/// What color the text should be.
///
/// This will ignore [`DefaultFontColor`] and will stay the same size even if [`DefaultFontColor`]
/// changes.
#[derive(Component, Deref, Default)]
pub struct FontColor(pub Color);
impl FontColor {
    /// Creates a new [`FontColor`]
    pub fn new(value: impl Into<Color>) -> Self {
        Self(value.into())
    }
    /// Retrives internal [`Color`]
    pub fn into_inner(&self) -> Color {
        self.0
    }
}

// Font Descriptors

/// A marker component that indicates that a peice of text should be styled by the [`ReactiveFontPlugin`]. Text
/// without this marker will not be styled
#[derive(Component)]
pub struct ReactiveFont;

/// This font that a [`ReactiveFont`] is using. If this is not specified it will default to
/// [`DefaultFont`]
#[derive(Component, Debug)]
#[relationship(relationship_target = UsedBy)]
pub struct UsingFont(Entity);

// Font Collections

/// The default font to be used when one is not specified.
///
/// This doesn't need to be set, but a font will have to be manually specified for each peice of
/// text if not.
#[derive(Resource)]
pub struct DefaultFont(pub Entity);
impl DefaultFont {
    pub fn new(value: Entity) -> Self {
        Self(value)
    }
    pub fn into_inner(&self) -> Entity {
        self.0
    }
}

/// A collection of font information.
#[derive(Component)]
#[require(
    RegularFont,
    ItalicFont,
    BoldFont,
    BoldItalicFont,
    DefaultFontSize,
    DefaultFontColor,
    UsedBy
)]
pub struct FontCollection;

/// All the text that uses a specific [`FontCollection`]
#[derive(Component, Default)]
#[relationship_target(relationship = UsingFont)]
pub struct UsedBy(Vec<Entity>);

/// The regular font used by a [`FontCollection`]
#[derive(Component, Deref, Default)]
pub struct RegularFont(pub Handle<Font>);

/// The italic font used by a [`FontCollection`]
#[derive(Component, Deref, Default)]
pub struct ItalicFont(pub Handle<Font>);

/// The bold font used by a [`FontCollection`]
#[derive(Component, Deref, Default)]
pub struct BoldFont(pub Handle<Font>);

/// The bold-italic font used by a [`FontCollection`]
#[derive(Component, Deref, Default)]
pub struct BoldItalicFont(pub Handle<Font>);

/// The default font size for a [`FontCollection`]
#[derive(Component)]
pub struct DefaultFontSize(pub f32);
impl Default for DefaultFontSize {
    fn default() -> Self {
        Self(15.)
    }
}
impl DefaultFontSize {
    /// Creates a new [`DefaultFontSize`]
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    /// Retrives the internal size
    pub fn into_inner(&self) -> f32 {
        self.0
    }
}

/// The default font color for a [`FontCollection`]
#[derive(Component, Default)]
pub struct DefaultFontColor(pub Color);
impl DefaultFontColor {
    /// Creates a new [`DefaultFontColor`]
    pub fn new(value: impl Into<Color>) -> Self {
        Self(value.into())
    }
    /// Retrives the internal [`Color`]
    pub fn into_inner(&self) -> Color {
        self.0
    }
}
