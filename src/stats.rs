use super::models::{KillCount, MeansOfDeath};
use super::round::QuakeRound;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct RoundStats {
    round_number: u32,
    total_kills: i32,
    players: Vec<String>,
    kills: HashMap<String, KillCount>,
    kills_by_means: HashMap<MeansOfDeath, u32>,
}

impl From<QuakeRound> for RoundStats {
    fn from(game_stats: QuakeRound) -> Self {
        let players = game_stats.get_players_names();
        let kills = game_stats.get_kills();
        let kills_by_means = game_stats.get_kills_by_means();
        let total_kills = game_stats.get_total_kills();
        let round_number: u32 = game_stats.get_round_number();
        RoundStats {
            round_number,
            total_kills,
            players,
            kills,
            kills_by_means,
        }
    }
}
