pub use bevy::prelude::*;
pub use tilesetanims;

mod loader;
mod systems;

use bevy::{reflect::TypeUuid, utils::HashMap};
use serde::Deserialize;
use tilesetanims::TilesetAnims;

/// Bevy Plugin for adding 2D characters
///
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_plugin(tilesetanims::TilesetAnimPlugin)
            .add_asset::<Character>()
            .init_asset_loader::<loader::CharacterLoader>()
            .add_system(systems::character_builder.system())
            .add_system(systems::character_anim_sync.system());
    }
}

// Character Handle
#[derive(Deserialize, TypeUuid)]
#[uuid = "9fa5febb-1a7b-4864-9534-2d5df8df82f4"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub name: Option<String>,
    pub sprite_sheet: CharacterSpriteSheet,
    pub anims: HashMap<String, Vec<u32>>,
    pub layers: Option<Vec<CharacterSprite>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CharacterSprite {
    sprite_sheet: CharacterSpriteSheet,
    anims: HashMap<String, Vec<u32>>,
    #[serde(default)]
    offset: Vec3,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CharacterSpriteSheet {
    pub path: String,
    pub grid_size: Vec2,
    pub rows: usize,
    pub columns: usize,
}

#[derive(Clone)]
pub struct CharacterCurrentAnim(pub String);
impl CharacterCurrentAnim {
    pub fn set(&mut self, anim: &str) {
        self.0 = anim.into()
    }
}

impl Into<CharacterCurrentAnim> for &str {
    fn into(self) -> CharacterCurrentAnim {
        CharacterCurrentAnim(self.to_string())
    }
}
impl Into<CharacterCurrentAnim> for String {
    fn into(self) -> CharacterCurrentAnim {
        CharacterCurrentAnim(self)
    }
}

pub struct CharacterLayer;

/// A unit-like struct for defining whether a character is fully loaded yet.
pub struct LoadedCharacter;

// Bundle
#[derive(Bundle)]
pub struct CharacterBundle {
    pub character: Handle<Character>,
    pub current_anim: CharacterCurrentAnim,
    /// Timer duration defines how long before next anim frame.
    pub timer: Timer,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            character: Default::default(),
            current_anim: TilesetAnims::default().current_anim.into(),
            timer: Timer::from_seconds(0.1, true),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
