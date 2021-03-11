use super::*;
use super::tilesetanims::TilesetAnims;

/// Sets the animation of the character layers to the same as the
/// `CharacterCurrentAnim` / `Handle<Character>` created by the CharacterBundle
pub fn character_anim_sync(
    character: Query<(&CharacterCurrentAnim, &Children), With<Handle<Character>>>,
    mut layers: Query<&mut TilesetAnims, With<CharacterLayer>>,
) {
    for (canim, children) in character.iter() {
        for child in children.iter() {
            if let Ok(mut layer) = layers.get_mut(*child) {
                if let Some(_anim) = layer.anims.get(&canim.0) {
                    layer.set_anim(canim.0.as_str())
                } else { layer.set_anim("default") }
            }
        }
    }
}

pub fn character_builder(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    characters: Res<Assets<Character>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(&Handle<Character>, &CharacterCurrentAnim, &Timer, Entity), Without<LoadedCharacter>>,
) {
    for (character_handle, current_anim, timer, entity) in query.iter() {
        if let Some(character) = characters.get(character_handle) {
            // Closure //
            // Returns the Handle<TextureAtlas> from path.
            let mut load_atlas_handle =
                |path: &str, grid_size: Vec2, columns: usize, rows: usize| {
                    let texture_handle = asset_server.load(path);
                    let texture_atlas =
                        TextureAtlas::from_grid(texture_handle, grid_size, columns, rows);
                    texture_atlases.add(texture_atlas)
                };

            // Main-Layer //
            let mainlayer = commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: load_atlas_handle(
                        character.sprite_sheet.path.as_str(),
                        character.sprite_sheet.grid_size,
                        character.sprite_sheet.columns,
                        character.sprite_sheet.rows,
                    ),
                    ..Default::default()
                })
                .with(TilesetAnims {
                    anims: character.anims.clone(),
                    current_anim: current_anim.0.clone(),
                    timer: timer.clone(),
                    ..Default::default()
                })
                .with(CharacterLayer)
                .current_entity()
                .unwrap();
            commands.push_children(entity, &[mainlayer]);

            // Sub-Layers //
            if let Some(layers) = &character.layers {
                for layer in layers.iter() {
                    let sublayers = commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: load_atlas_handle(
                                layer.sprite_sheet.path.as_str(),
                                layer.sprite_sheet.grid_size,
                                layer.sprite_sheet.columns,
                                layer.sprite_sheet.rows,
                            ),
                            transform: Transform::from_translation(layer.offset),
                            ..Default::default()
                        })
                        .with(TilesetAnims {
                            anims: layer.anims.clone(),
                            timer: timer.clone(),
                            ..Default::default()
                        })
                        .with(CharacterLayer)
                        .current_entity()
                        .unwrap();
                    commands.push_children(entity, &[sublayers]);
                }
            }
            // Mark as LoadedCharacter
            commands.insert_one(entity, LoadedCharacter);
        }
    }
}
