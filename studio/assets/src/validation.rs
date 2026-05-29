use crate::assets::{browser, catalog, import};
pub fn validate_asset_browser() -> crate::CreatorDiagnostic { crate::diagnostic("asset-browser", &["import", "preview", "catalog", "package-membership"] ) }
pub fn asset_pipeline_equivalence() -> bool { let a = import::import_asset("hero", b"pixel", "image"); let c = catalog::catalog_hash(&[a.clone()]); a.compatible && browser::preview_asset(&a.asset_hash) == browser::preview_asset(&a.asset_hash) && c == catalog::catalog_hash(&[a]) }
