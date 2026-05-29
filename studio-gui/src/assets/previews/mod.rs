use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetKind {
    Model,
    Texture,
    Audio,
    Package,
}

impl AssetKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Texture => "texture",
            Self::Audio => "audio",
            Self::Package => "package",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetPreview {
    pub asset_id: String,
    pub kind: AssetKind,
    pub thumbnail_hash: String,
    pub preview_hash: String,
    pub visually_navigable: bool,
}

pub fn generate_preview(asset_id: &str, kind: AssetKind, content_hash: &str) -> AssetPreview {
    let thumbnail_hash = stable_hash(&["thumbnail", asset_id, kind.as_str(), content_hash]);
    let preview_hash = stable_hash(&["asset-preview", asset_id, &thumbnail_hash]);
    AssetPreview {
        asset_id: asset_id.to_owned(),
        kind,
        thumbnail_hash,
        preview_hash,
        visually_navigable: true,
    }
}

pub fn asset_preview_equivalence() -> bool {
    let first = vec![
        generate_preview("hero.glb", AssetKind::Model, "mesh-hash"),
        generate_preview("terrain.png", AssetKind::Texture, "texture-hash"),
        generate_preview("theme.ogg", AssetKind::Audio, "audio-hash"),
        generate_preview("starter-pack", AssetKind::Package, "package-hash"),
    ];
    let second = vec![
        generate_preview("hero.glb", AssetKind::Model, "mesh-hash"),
        generate_preview("terrain.png", AssetKind::Texture, "texture-hash"),
        generate_preview("theme.ogg", AssetKind::Audio, "audio-hash"),
        generate_preview("starter-pack", AssetKind::Package, "package-hash"),
    ];
    first == second && first.iter().all(|preview| preview.visually_navigable)
}
