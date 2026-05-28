use super::player::GameplayPlayer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplaySession {
    pub session_id: String,
    pub continuity_root: String,
    pub players: Vec<GameplayPlayer>,
}

impl GameplaySession {
    pub fn new(session_id: impl Into<String>) -> Self {
        let session_id = session_id.into();
        Self {
            continuity_root: format!("root:everarcade:gameplay:{session_id}:v1"),
            session_id,
            players: Vec::new(),
        }
    }

    pub fn attach_player(&mut self, player_id: impl Into<String>) {
        let player = GameplayPlayer::new(player_id, &self.session_id);
        if !self.players.iter().any(|p| p.player_id == player.player_id) {
            self.players.push(player);
            self.players.sort();
        }
    }
}
