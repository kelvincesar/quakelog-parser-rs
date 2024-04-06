use serde::Serialize;

pub const SPLIT_MARKER: &str = ": ";
pub const IGNORE_MARKER: &str = "------------------------------------------------------------";
pub const WORLD_ID: PlayerId = 1022;

pub type KillCount = i32;
pub type PlayerId = u32;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    kills: KillCount,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            kills: 0,
        }
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_kills(&self) -> KillCount {
        self.kills
    }

    pub fn increment_kills(&mut self) {
        self.kills += 1;
    }

    pub fn decrement_kills(&mut self) {
        self.kills -= 1;
    }
}

#[derive(Debug, PartialEq)]
pub enum QuakeAction {
    InitGame,
    Exit,
    ClientConnect,
    ClientDisconnect,
    ClientUserInfoChanged,
    ClientBegin,
    ShutdownGame,
    Item,
    Kill,
    Score,
    Say,
}
impl QuakeAction {
    pub fn from_str(action: &str) -> Option<Self> {
        let action = match action {
            "InitGame" => QuakeAction::InitGame,
            "Exit" => QuakeAction::Exit,
            "ClientConnect" => QuakeAction::ClientConnect,
            "ClientDisconnect" => QuakeAction::ClientDisconnect,
            "ClientUserinfoChanged" => QuakeAction::ClientUserInfoChanged,
            "ClientBegin" => QuakeAction::ClientBegin,
            "ShutdownGame:" | "ShutdownGame" => QuakeAction::ShutdownGame,
            "Item" => QuakeAction::Item,
            "Kill" => QuakeAction::Kill,
            "score" => QuakeAction::Score,
            "say" => QuakeAction::Say,
            _ => return None,
        };

        Some(action)
    }
}

#[warn(clippy::enum_variant_names)]
#[derive(Serialize, PartialEq, Eq, Hash, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeansOfDeath {
    ModUnknown,
    ModShotgun,
    ModGauntlet,
    ModMachinegun,
    ModGrenade,
    ModGrenadeSplash,
    ModRocket,
    ModRocketSplash,
    ModPlasma,
    ModPlasmaSplash,
    ModRailgun,
    ModLightning,
    ModBfg,
    ModBfgSplash,
    ModWater,
    ModSlime,
    ModLava,
    ModCrush,
    ModTelefrag,
    ModFalling,
    ModSuicide,
    ModTargetLaser,
    ModTriggerHurt,
    ModNail,
    ModChaingun,
    ModProximityMine,
    ModKamikaze,
    ModJuiced,
    ModGrapple,
}

impl MeansOfDeath {
    pub fn from_str(mean_of_death: &str) -> Self {
        match mean_of_death {
            "MOD_UNKNOWN" => MeansOfDeath::ModUnknown,
            "MOD_SHOTGUN" => MeansOfDeath::ModShotgun,
            "MOD_GAUNTLET" => MeansOfDeath::ModGauntlet,
            "MOD_MACHINEGUN" => MeansOfDeath::ModMachinegun,
            "MOD_GRENADE" => MeansOfDeath::ModGrenade,
            "MOD_GRENADE_SPLASH" => MeansOfDeath::ModGrenadeSplash,
            "MOD_ROCKET" => MeansOfDeath::ModRocket,
            "MOD_ROCKET_SPLASH" => MeansOfDeath::ModRocketSplash,
            "MOD_PLASMA" => MeansOfDeath::ModPlasma,
            "MOD_PLASMA_SPLASH" => MeansOfDeath::ModPlasmaSplash,
            "MOD_RAILGUN" => MeansOfDeath::ModRailgun,
            "MOD_LIGHTNING" => MeansOfDeath::ModLightning,
            "MOD_BFG" => MeansOfDeath::ModBfg,
            "MOD_BFG_SPLASH" => MeansOfDeath::ModBfgSplash,
            "MOD_WATER" => MeansOfDeath::ModWater,
            "MOD_SLIME" => MeansOfDeath::ModSlime,
            "MOD_LAVA" => MeansOfDeath::ModLava,
            "MOD_CRUSH" => MeansOfDeath::ModCrush,
            "MOD_TELEFRAG" => MeansOfDeath::ModTelefrag,
            "MOD_FALLING" => MeansOfDeath::ModFalling,
            "MOD_SUICIDE" => MeansOfDeath::ModSuicide,
            "MOD_TARGET_LASER" => MeansOfDeath::ModTargetLaser,
            "MOD_TRIGGER_HURT" => MeansOfDeath::ModTriggerHurt,
            "MOD_NAIL" => MeansOfDeath::ModNail,
            "MOD_CHAINGUN" => MeansOfDeath::ModChaingun,
            "MOD_PROXIMITY_MINE" => MeansOfDeath::ModProximityMine,
            "MOD_KAMIKAZE" => MeansOfDeath::ModKamikaze,
            "MOD_JUICED" => MeansOfDeath::ModJuiced,
            "MOD_GRAPPLE" => MeansOfDeath::ModGrapple,
            _ => MeansOfDeath::ModUnknown,
        }
    }
}
