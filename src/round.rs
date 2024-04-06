use super::models::{KillCount, MeansOfDeath, Player, PlayerId, WORLD_ID};
use super::parser::QuakeLogLine;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct QuakeRound {
    round_number: u32,
    total_kills: KillCount,
    players: HashMap<PlayerId, Player>,
    kills_by_means: HashMap<MeansOfDeath, u32>,
    started: bool,
}

impl QuakeRound {
    pub fn new(round_number: u32) -> Self {
        QuakeRound {
            round_number,
            total_kills: 0,
            players: HashMap::new(),
            kills_by_means: HashMap::new(),
            started: false,
        }
    }

    pub fn start_round(&mut self) {
        self.started = true;
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn process_client_user_info_changed(&mut self, line: &QuakeLogLine) -> bool {
        let (id, name) = match line.parse_user_change_info() {
            Some((id, name)) => (id, name),
            None => return false,
        };

        self.update_or_insert_player(id, name);
        true
    }

    pub fn process_kill(&mut self, line: &QuakeLogLine) -> bool {
        let (killer_id, killed_id, means_of_death) = match line.parse_kill_info() {
            Some(info) => info,
            None => return false,
        };
        // "When `<world>` kill a player, that player loses -1 kill score."
        if killer_id == WORLD_ID || killer_id == killed_id {
            // decrement player kills
            if let Some(player) = self.players.get_mut(&killed_id) {
                player.decrement_kills();
            }
        } else {
            // increment player kills
            if let Some(player) = self.players.get_mut(&killer_id) {
                player.increment_kills();
            }
        }
        self.total_kills += 1;
        *self.kills_by_means.entry(means_of_death).or_insert(0) += 1;
        true
    }

    fn update_or_insert_player(&mut self, id: PlayerId, name: String) {
        match self.players.entry(id) {
            Entry::Occupied(mut entry) => entry.get_mut().update_name(&name),
            Entry::Vacant(entry) => {
                let player = Player::new(&name);
                entry.insert(player);
            }
        };
    }
    pub fn get_total_kills(&self) -> KillCount {
        self.total_kills
    }

    pub fn get_players_names(&self) -> Vec<String> {
        self.players
            .values()
            .map(|player| player.get_name().to_string())
            .collect()
    }

    pub fn get_kills(&self) -> HashMap<String, KillCount> {
        self.players
            .values()
            .map(|player| (player.get_name().to_string(), player.get_kills()))
            .collect()
    }

    pub fn get_kills_by_means(&self) -> HashMap<MeansOfDeath, u32> {
        self.kills_by_means.clone()
    }

    pub fn get_round_number(&self) -> u32 {
        self.round_number
    }
}
