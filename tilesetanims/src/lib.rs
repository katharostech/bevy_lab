use bevy::prelude::*;
use bevy::utils::HashMap;

// Plugin //

pub struct TilesetAnimPlugin;

impl Plugin for TilesetAnimPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_system_to_stage(stage::POST_UPDATE, multi_anim_tileset_animator.system())
            .add_system_to_stage(stage::POST_UPDATE, single_anim_tileset_animator.system());
    }
}

// Structure //

/// Specifies a single animation for an atlas with a tuple.
///
/// (`0`: frame indexes, `1`: frame duration)
pub struct TilesetAnim(pub Vec<u32>, pub f32);

pub struct TilesetAnims {
    pub anims: HashMap<String, Vec<u32>>,
    pub current_anim: String,
    pub timer: Timer,
    pub current_anim_index: u32,
}
impl TilesetAnims {
    pub fn set_anim(&mut self, anim: &str) {
        if anim.to_string() != self.current_anim {
            let duration = self.timer.duration();
            self.timer.set_elapsed(duration);
            self.current_anim_index = 0;
            self.current_anim = anim.into();
        }
    }
}

// Defaults //

impl Default for TilesetAnim {
    fn default() -> Self {
        Self(vec![0], 0.1)
    }
}
impl Default for TilesetAnims {
    fn default() -> Self {
        Self {
            anims: HashMap::default(),
            current_anim: String::from("default"),
            timer: Timer::from_seconds(0.1, true),
            current_anim_index: 0,
        }
    }
}

// Systems //

/// Animates the TextureAtlasSprite's using their TilesetAnims struct data.
pub fn multi_anim_tileset_animator(
    time: Res<Time>,
    mut query: Query<(
        &mut TilesetAnims,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut tileanims, mut sprite) in query.iter_mut() {
        tileanims.timer.tick(time.delta_seconds());
        if tileanims.timer.finished() {
            let TilesetAnims {
                current_anim,
                current_anim_index,
                timer: _,
                anims,
            } = &mut *tileanims;
            if let Some(current_anim_vec) = anims.get(current_anim) {                     // Check Anim Existence
                sprite.index = match current_anim_vec.get(*current_anim_index as usize) { // Set Frame
                    Some(frame) => *frame,
                    None => {
                        // This warning likely means that a plugin user has manually set the current_anim_index
                        // to a larger number than the amount of frames in the current_anim.
                        warn!("An animation index has gone out of \"{}\"'s bounds. Setting to zero...", current_anim);
                        0
                    }
                };
                *current_anim_index = current_anim_index.wrapping_add(1);                  // Cycle Anim Index
                *current_anim_index = *current_anim_index % current_anim_vec.len() as u32; // Wrap Index Back Around
            } else {
                warn!("Animation '{}' Does Not Exist For Sprite", current_anim);
                if let Some(_anim) = anims.get("default".into()) { *current_anim = "default".into() }
                else { sprite.index = 0 }
            }
        }
    }
}

/// Animates the TextureAtlasSprite's using their TilesetAnim struct data.
pub fn single_anim_tileset_animator(
    time: Res<Time>,
    commands: &mut Commands,
    mut query: Query<(
        &mut TilesetAnim,
        &mut TextureAtlasSprite,
        Option<&mut Timer>,
        Entity,
    )>,
) {
    for (tileanim, mut sprite, timer, entity) in query.iter_mut() {

        if let Some(mut timer) = timer {
            timer.tick(time.delta_seconds());
            if timer.finished() {
                sprite.index = ((sprite.index as usize + 1) % tileanim.0.len()) as u32;
            }
        } else {
            let mut new_timer = Timer::from_seconds(tileanim.1, true);
            new_timer.tick(time.delta_seconds());
            if new_timer.finished() {
                sprite.index = ((sprite.index as usize + 1) % tileanim.0.len()) as u32;
            }
            commands.insert_one(entity, new_timer);
        }
    }
}
