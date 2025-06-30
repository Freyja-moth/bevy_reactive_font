use bevy::{
    color::palettes::css,
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};
use bevy_reactive_font::prelude::*;

const GREEN: Color = Color::srgb_u8(162, 209, 133);
const LIME: Color = Color::srgb_u8(133, 209, 155);
const BLUE: Color = Color::srgb_u8(133, 209, 209);
const DEEP_BLUE: Color = Color::srgb_u8(133, 150, 209);
const PURPLE: Color = Color::srgb_u8(191, 133, 209);
const PINK: Color = Color::srgb_u8(209, 133, 168);
const BACKGROUND: Color = Color::srgb_u8(15, 42, 63);

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct LiberationMono;

#[derive(Component)]
pub struct OpenDyslexic;

#[derive(Resource, Default)]
pub enum CurrentFontColor {
    Green,
    Lime,
    Blue,
    DeepBlue,
    #[default]
    Purple,
    Pink,
}
impl CurrentFontColor {
    pub fn cycle(&mut self) -> &mut Self {
        *self = match self {
            Self::Green => Self::Lime,
            Self::Lime => Self::Blue,
            Self::Blue => Self::DeepBlue,
            Self::DeepBlue => Self::Purple,
            Self::Purple => Self::Pink,
            Self::Pink => Self::Green,
        };
        self
    }
    pub fn cycle_back(&mut self) -> &mut Self {
        *self = match self {
            Self::Green => Self::Lime,
            Self::Lime => Self::Blue,
            Self::Blue => Self::DeepBlue,
            Self::DeepBlue => Self::Purple,
            Self::Purple => Self::Pink,
            Self::Pink => Self::Green,
        };
        self
    }
    pub fn into_color(&self) -> Color {
        match self {
            Self::Green => GREEN,
            Self::Lime => LIME,
            Self::Blue => BLUE,
            Self::DeepBlue => DEEP_BLUE,
            Self::Purple => PURPLE,
            Self::Pink => PINK,
        }
    }
}

pub fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, ReactiveFontPlugin))
        .init_resource::<CurrentFontColor>()
        .insert_resource(ClearColor(BACKGROUND))
        .add_systems(Startup, (spawn_camera, spawn_fonts, spawn_text).chain())
        .add_systems(
            Update,
            (
                increase_default_font_size.run_if(input_pressed(KeyCode::ArrowUp)),
                decrease_default_font_size.run_if(input_pressed(KeyCode::ArrowDown)),
                cycle_default_color_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                cycle_default_color_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
                update_default_color.run_if(resource_changed::<CurrentFontColor>),
            ),
        )
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let regular_font = asset_server.load("fonts/liberation_mono/regular.ttf");
    let italic_font = asset_server.load("fonts/liberation_mono/italic.ttf");
    let bold_font = asset_server.load("fonts/liberation_mono/bold.ttf");
    let bold_italic_font = asset_server.load("fonts/liberation_mono/bold_italic.ttf");

    let default_font = commands
        .spawn((
            FontCollection,
            LiberationMono,
            RegularFont(regular_font),
            ItalicFont(italic_font),
            BoldFont(bold_font),
            BoldItalicFont(bold_italic_font),
            DefaultFontSize(20.),
            DefaultFontColor::new(PURPLE),
        ))
        .id();

    commands.insert_resource(DefaultFont::new(default_font));

    let regular_font = asset_server.load("fonts/opendyslexic/regular.otf");
    let italic_font = asset_server.load("fonts/opendyslexic/italic.otf");
    let bold_font = asset_server.load("fonts/opendyslexic/bold.otf");
    let bold_italic_font = asset_server.load("fonts/opendyslexic/bold_italic.otf");

    commands.spawn((
        FontCollection,
        OpenDyslexic,
        RegularFont(regular_font),
        ItalicFont(italic_font),
        BoldFont(bold_font),
        BoldItalicFont(bold_italic_font),
        DefaultFontSize(20.),
        DefaultFontColor::new(PURPLE),
    ));
}

fn spawn_text(
    mut commands: Commands,
    camera: Single<Entity, With<Camera2d>>,
    open_dyslexic: Single<Entity, With<OpenDyslexic>>,
) {
    let fonted_font = commands
        .spawn((
            ReactiveFont,
            Text::new("Wow this took way too long."),
            FontColor::new(css::REBECCA_PURPLE),
        ))
        .id();

    commands
        .entity(*open_dyslexic)
        .add_one_related::<UsingFont>(fonted_font);

    commands
        .spawn((
            UiRoot,
            UiTargetCamera(*camera),
            Node {
                row_gap: Val::Percent(2.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            children![
                (Text::new("Hello there"), ReactiveFont),
                (Text::new("I did a cool thing!"), Bold, Italic, ReactiveFont),
                (
                    Text::new("And came up with a way of storing fonts."),
                    Bold,
                    ReactiveFont
                ),
                (
                    Text::new("They're stored in a table with RegularFont, BoldFont, ItalicFont, and BoldItalicFont components."),
                    ReactiveFont,
                ),
                (
                    Text::new("You can even set the default font color and size with DefaultFontColor and DefaultFontSize"),
                    ReactiveFont
                ),
                (
                    Text::new("And then override them with FontColor and FontSize on the actual text"),
                    FontSize(25.),
                    FontColor::new(Color::srgb_u8(94, 145, 136)),
                    ReactiveFont,
                ),
                (
                    Text::new("A default font can be set, and you can use relationships to specificy what text uses what font"),
                    ReactiveFont,
                )
            ],
        ))
        .add_child(fonted_font);

    commands.spawn((
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            (
                Text::new("Press the up and down arrows to increase the default text size"),
                FontSize::new(15.),
                FontColor::new(PURPLE),
                ReactiveFont
            ),
            (
                Text::new("And the right and left arrows to change the default font color"),
                FontSize::new(15.),
                FontColor::new(PURPLE),
                ReactiveFont
            )
        ],
    ));
}

fn increase_default_font_size(mut font_size: Query<&mut DefaultFontSize>) {
    font_size.iter_mut().for_each(|mut font_size| {
        font_size.0 += 1.;
    })
}

fn decrease_default_font_size(mut font_size: Query<&mut DefaultFontSize>) {
    font_size.iter_mut().for_each(|mut font_size| {
        font_size.0 -= 1.;
    })
}

fn cycle_default_color_right(mut current_font_color: ResMut<CurrentFontColor>) {
    current_font_color.cycle();
}

fn cycle_default_color_left(mut current_font_color: ResMut<CurrentFontColor>) {
    current_font_color.cycle_back();
}

fn update_default_color(
    mut default_font_color: Query<&mut DefaultFontColor>,
    current_font_color: Res<CurrentFontColor>,
) {
    default_font_color.iter_mut().for_each(|mut color| {
        color.0 = current_font_color.into_color();
    });
}
