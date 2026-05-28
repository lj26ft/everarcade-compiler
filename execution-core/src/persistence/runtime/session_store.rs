use crate::gameplay::GameplaySession;
#[derive(Clone, Debug, Default)]
pub struct RuntimeSessionStore {
    pub sessions: Vec<GameplaySession>,
}
impl RuntimeSessionStore {
    pub fn persist(&mut self, session: GameplaySession) {
        self.sessions.push(session);
    }
}
