use super::models::{MeansOfDeath, PlayerId, QuakeAction, IGNORE_MARKER, SPLIT_MARKER};

fn parse_time_and_action(time_and_action: &str) -> Option<(String, QuakeAction)> {
    let parts: Vec<&str> = time_and_action.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let time = parts[0].to_string();
    let action = QuakeAction::from_str(parts[1])?;
    Some((time, action))
}

#[derive(Debug, PartialEq)]
pub struct QuakeLogLine {
    pub time: String,
    pub action: QuakeAction,
    pub info: String,
}

impl QuakeLogLine {
    pub fn from_str(line: &str) -> Option<Self> {
        if line.contains(IGNORE_MARKER) {
            return None;
        }

        // use of unwrap for the case of "ShutdownGame" line
        let (time_and_action, info) = line.split_once(SPLIT_MARKER).unwrap_or((line, ""));

        // parse the first part of the line which may have the time and action
        if let Some((time, action)) = parse_time_and_action(time_and_action) {
            return Some(QuakeLogLine {
                time,
                action,
                info: info.to_string(),
            });
        }

        None
    }

    pub fn get_action(&self) -> &QuakeAction {
        &self.action
    }

    /// parse lines ClientUserinfoChanged: <client_id> n\<name>\t\...
    pub fn parse_user_change_info(&self) -> Option<(PlayerId, String)> {
        let (id_str, change_info) = self.info.split_once(" n\\")?;
        let id = id_str.parse::<PlayerId>().ok()?;
        let name = change_info.split_once("\\t")?.0.to_string();
        Some((id, name))
    }

    /// parse lines Kill: <killer_id> <killed_id> <death_id>: <killer name> killed <killed name> by <means of death>
    pub fn parse_kill_info(&self) -> Option<(PlayerId, PlayerId, MeansOfDeath)> {
        let (kill_ids, kill_log) = self.info.split_once(SPLIT_MARKER)?;
        let mut ids = kill_ids.split_whitespace();
        // parse killer_id
        let killer_id = match ids.next().and_then(|s| s.parse::<PlayerId>().ok()) {
            Some(id) => id,
            None => return None,
        };

        // parse killed_id
        let killed_id = match ids.next().and_then(|s| s.parse::<PlayerId>().ok()) {
            Some(id) => id,
            None => return None,
        };

        // cannot use the death_id because of the #ifdef on c enum
        let means_of_death = match kill_log.split_once(" by ") {
            Some((_, means_of_death)) => means_of_death,
            None => {
                eprintln!("Invalid means of death: {}", kill_log);
                return None;
            }
        };

        let means_of_death = MeansOfDeath::from_str(means_of_death);
        Some((killer_id, killed_id, means_of_death))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const USER_INFO_CHANGE_OK: &str = r#"3:49 ClientUserinfoChanged: 5 n\Assasinu Credi\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0"#;
    const KILL_INFO_OK: &str = "2:29 Kill: 3 4 10: Isgalamido killed Zeh by MOD_RAILGUN";
    #[test]
    fn quake_line_with_ignore_marker() {
        let line = super::IGNORE_MARKER;
        assert_eq!(QuakeLogLine::from_str(line), None);
    }

    #[test]
    fn quake_line_with_kill_log() {
        let line = KILL_INFO_OK;
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.time, "2:29");
        assert_eq!(quake_line.action, QuakeAction::Kill);
        assert_eq!(
            quake_line.info,
            "3 4 10: Isgalamido killed Zeh by MOD_RAILGUN"
        );
    }

    #[test]
    fn quake_line_with_user_info_change() {
        let line = USER_INFO_CHANGE_OK;
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.time, "3:49");
        assert_eq!(quake_line.action, QuakeAction::ClientUserInfoChanged);
        assert_eq!(
            quake_line.info,
            r#"5 n\Assasinu Credi\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0"#
        );
    }

    #[test]
    fn quake_parse_invalid_line() {
        let line = "2:29 Kill 3 4 10: Isgalamido killed Zeh by MOD_RAILGUN";
        assert_eq!(QuakeLogLine::from_str(line), None);
    }

    #[test]
    fn quake_parse_shutdown_game() {
        let line = " 1:47 ShutdownGame:";
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.time, "1:47");
        assert_eq!(quake_line.action, QuakeAction::ShutdownGame);
        assert_eq!(quake_line.info, "");
    }

    #[test]
    fn quake_line_parse_user_change_info_ok() {
        let line = USER_INFO_CHANGE_OK;
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        let (user_id, user_name) = quake_line.parse_user_change_info().unwrap();
        assert_eq!(user_id, 5);
        assert_eq!(user_name, "Assasinu Credi");
    }

    #[test]
    fn quake_line_parse_user_change_info_invalid_id() {
        let line = "3:49 ClientUserinfoChanged: id n\\Assasinu Credi\\t\\0\\model\\sarge";
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.parse_user_change_info(), None);
    }

    #[test]
    fn quake_line_parse_user_change_info_invalid_name() {
        let line = "3:49 ClientUserinfoChanged: 2 n\\Assasinu Credi\\0\\model\\sarge";
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.parse_user_change_info(), None);
    }

    #[test]
    fn quake_line_parse_kill_info_ok() {
        let line = KILL_INFO_OK;
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        let (killer_id, killed_id, means_of_death) = quake_line.parse_kill_info().unwrap();
        assert_eq!(killer_id, 3);
        assert_eq!(killed_id, 4);
        assert_eq!(means_of_death, MeansOfDeath::ModRailgun);
    }

    #[test]
    fn quake_line_parse_kill_info_invalid_killer_id() {
        let line = "2:29 Kill: id 4 10: Isgalamido killed Zeh by MOD_RAILGUN";
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.parse_kill_info(), None);
    }

    #[test]
    fn quake_line_parse_kill_info_invalid_killed_id() {
        let line = "2:29 Kill: 3 id 10: Isgalamido killed Zeh by MOD_RAILGUN";
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        assert_eq!(quake_line.parse_kill_info(), None);
    }

    #[test]
    fn quake_line_parse_kill_info_invalid_means_of_death() {
        let line = "2:29 Kill: 3 4 10: Isgalamido killed Zeh by Morte";
        let quake_line = QuakeLogLine::from_str(line).unwrap();
        let (killer_id, killed_id, means_of_death) = quake_line.parse_kill_info().unwrap();
        assert_eq!(killer_id, 3);
        assert_eq!(killed_id, 4);
        assert_eq!(means_of_death, MeansOfDeath::ModUnknown);
    }
}
