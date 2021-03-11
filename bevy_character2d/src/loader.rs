use super::Character;
use bevy::asset::{AssetLoader, LoadedAsset};

#[derive(Default)]
pub struct CharacterLoader;

impl AssetLoader for CharacterLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move { Ok(load_character(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &["character.yml", "character.yaml"]
    }
}

#[derive(thiserror::Error, Debug)]
enum CharacterLoaderError {
    #[error("Could not parse character file: {0}")]
    DeserializationError(#[from] serde_yaml::Error),
}

async fn load_character<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<(), CharacterLoaderError> {
    // Load the character
    let character: Character = serde_yaml::from_slice(bytes)?;

    // Set the character asset
    load_context.set_default_asset(LoadedAsset::new(character));

    Ok(())
}
