use crate::{
    error::SdkError, game::DeterministicGame, input::PlayerInput, runtime::DeterministicRuntime,
};

pub fn validate_equivalence<G: DeterministicGame + Default>(
    inputs: Vec<PlayerInput>,
) -> Result<String, SdkError> {
    let mut a = DeterministicRuntime::new(G::default());
    let mut b = DeterministicRuntime::new(G::default());
    let ah = a.tick(inputs.clone())?;
    let bh = b.tick(inputs)?;
    if ah == bh {
        Ok(ah)
    } else {
        Err(SdkError::DivergenceDetected {
            expected: ah,
            actual: bh,
        })
    }
}

pub fn reject_authority_mutation<G: DeterministicGame>(game: G) -> bool {
    let mut rt = DeterministicRuntime::with_authority(game, false);
    matches!(
        rt.tick(Vec::new()),
        Err(SdkError::UnauthorizedAuthorityMutation)
    )
}
