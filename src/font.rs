use bevy::{asset::AsAssetId, prelude::*};

use crate::persistent_relationship_source::NeverEmptyVec;

/// Marks that a peice of text should be italic
#[derive(Component, Reflect)]
pub struct Italic;

/// Marks that a peice of text should be bold
#[derive(Component, Reflect)]
pub struct Bold;

/// How large the text should be.
///
/// This will ignore [`DefaultFontSize`] and will stay the same size even if [`DefaultFontSize`]
/// changes.
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Clone, Debug)]
pub struct FontSize(pub f32);
impl From<f32> for FontSize {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
impl Default for FontSize {
    fn default() -> Self {
        Self::new(15.)
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
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Clone, Default, Debug)]
pub struct FontColor(pub Color);
impl<C: Into<Color>> From<C> for FontColor {
    fn from(value: C) -> Self {
        Self::new(value)
    }
}
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
#[derive(Component, Reflect)]
pub struct ReactiveFont;

/// This font that a [`ReactiveFont`] is using. If this is not specified it will default to
/// [`DefaultFont`]
#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = UsedBy)]
pub struct UsingFont(pub Entity);

// Font Collections

/// The default font to be used when one is not specified.
///
/// This doesn't need to be set, but a font will have to be manually specified for each peice of
/// text if not.
#[derive(Resource, Reflect, Debug)]
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
#[derive(Component, Reflect)]
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
#[derive(Component, Reflect, Default, Debug)]
#[relationship_target(relationship = UsingFont)]
pub struct UsedBy(NeverEmptyVec<Entity>);

/// The regular font used by a [`FontCollection`]
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Eq, Clone, Default, Debug)]
pub struct RegularFont(pub Handle<Font>);
impl AsAssetId for RegularFont {
    type Asset = Font;
    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}
impl From<Handle<Font>> for RegularFont {
    fn from(value: Handle<Font>) -> Self {
        Self::new(value)
    }
}
impl RegularFont {
    pub fn new(value: Handle<Font>) -> Self {
        Self(value)
    }
    pub fn into_inner(&self) -> &Handle<Font> {
        &self.0
    }
}

/// The italic font used by a [`FontCollection`]
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Eq, Clone, Default, Debug)]
pub struct ItalicFont(pub Handle<Font>);
impl AsAssetId for ItalicFont {
    type Asset = Font;
    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}
impl From<Handle<Font>> for ItalicFont {
    fn from(value: Handle<Font>) -> Self {
        Self::new(value)
    }
}
impl ItalicFont {
    pub fn new(value: Handle<Font>) -> Self {
        Self(value)
    }
    pub fn into_inner(&self) -> &Handle<Font> {
        &self.0
    }
}

/// The bold font used by a [`FontCollection`]
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Eq, Clone, Default, Debug)]
pub struct BoldFont(pub Handle<Font>);
impl AsAssetId for BoldFont {
    type Asset = Font;
    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}
impl From<Handle<Font>> for BoldFont {
    fn from(value: Handle<Font>) -> Self {
        Self::new(value)
    }
}
impl BoldFont {
    pub fn new(value: Handle<Font>) -> Self {
        Self(value)
    }
    pub fn into_inner(&self) -> &Handle<Font> {
        &self.0
    }
}

/// The bold-italic font used by a [`FontCollection`]
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Eq, Clone, Default, Debug)]
pub struct BoldItalicFont(pub Handle<Font>);
impl AsAssetId for BoldItalicFont {
    type Asset = Font;
    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}
impl From<Handle<Font>> for BoldItalicFont {
    fn from(value: Handle<Font>) -> Self {
        Self::new(value)
    }
}
impl BoldItalicFont {
    pub fn new(value: Handle<Font>) -> Self {
        Self(value)
    }
    pub fn into_inner(&self) -> &Handle<Font> {
        &self.0
    }
}

/// The default font size for a [`FontCollection`]
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Clone, Debug)]
pub struct DefaultFontSize(pub f32);
impl From<f32> for DefaultFontSize {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
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
#[derive(Component, Reflect, DerefMut, Deref, PartialEq, Clone, Default, Debug)]
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
