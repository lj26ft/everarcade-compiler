pub type Hash = [u8; 32];

use super::{
    node_identity::CryptographicNodeIdentity, peer_fingerprint::derive_peer_fingerprint,
    signed_manifest::SignedIdentityManifest,
};

pub fn validate_identity(identity: &CryptographicNodeIdentity) -> bool {
    identity.fingerprint_root == derive_peer_fingerprint(identity.node_id, identity.public_key_root)
}

pub fn validate_manifest(manifest: &SignedIdentityManifest) -> bool {
    manifest.manifest_root != [0; 32]
        && manifest.signer_root != [0; 32]
        && manifest.signature_root != [0; 32]
}
