use crate::tank::Tank;
use std::fs;
use std::io;

const SAVE_FILE: &str = "tank.json";

pub fn save_tank(tank: &Tank) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tank)?;
    fs::write(SAVE_FILE, json)?;
    Ok(())
}

pub fn load_tank() -> Option<Tank> {
    if !std::path::Path::new(SAVE_FILE).exists() {
        return None;
    }

    let json = fs::read_to_string(SAVE_FILE).ok()?;
    serde_json::from_str(&json).ok()
}
