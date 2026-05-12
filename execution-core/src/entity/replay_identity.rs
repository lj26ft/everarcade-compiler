use super::identity::EntityIdentity;

pub fn reconstruct_identity_from_replay(seed: &[u8]) -> EntityIdentity {
    EntityIdentity::from_genesis(seed)
}
