use crate::gameplay::GameplayPlayer;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlayerRegistry {
    pub players: Vec<GameplayPlayer>,
}

impl PlayerRegistry {
    pub fn attach(&mut self, player: GameplayPlayer) {
        if !self.players.iter().any(|p| p.player_id == player.player_id) {
            self.players.push(player);
            self.players.sort();
        }
    }
}
