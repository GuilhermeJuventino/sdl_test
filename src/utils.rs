use std::collections::HashMap;

pub fn rad_to_deg(rad: f64) -> f64 {
    ( rad / ( 2.0 * std::f64::consts::PI )) * 360.0
}

// key manager functions
pub fn key_down(key_manager: &mut HashMap<String, bool>, keyname: String) {
    if !key_manager.contains_key(&keyname) {
        key_manager.entry(keyname).or_insert(true);
    } else {
        if let Some(x) = key_manager.get_mut(&keyname) {
            *x = true;
        }
    }
}

pub fn key_up(key_manager: &mut HashMap<String, bool>, keyname: String) {
    if !key_manager.contains_key(&keyname) {
        key_manager.entry(keyname).or_insert(false);
    } else {
        if let Some(x) = key_manager.get_mut(&keyname) {
            *x = false;
        }
    }
}

pub fn is_key_pressed(key_manager: &HashMap<String, bool>, value: &str) -> bool {
    key_manager.contains_key(&value.to_string()) && key_manager.get(&value.to_string()) == Some(&true)
}