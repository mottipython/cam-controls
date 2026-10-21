use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::v4l2::*;

pub fn get_config_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("cam-controls")
}

pub fn get_startup_file() -> PathBuf {
    get_config_dir().join("startup-default.json")
}

pub fn get_profiles_file() -> PathBuf {
    get_config_dir().join("profiles.json")
}

pub fn get_autostart_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("autostart").join("cam-controls-restore.desktop")
}

pub fn get_builtin_presets() -> HashMap<&'static str, HashMap<u32, i32>> {
    let mut presets = HashMap::new();

    let mut vivid = HashMap::new();
    vivid.insert(CID_SATURATION, 56);
    vivid.insert(CID_CONTRAST, 42);
    vivid.insert(CID_BRIGHTNESS, 128);
    vivid.insert(CID_SHARPNESS, 28);
    vivid.insert(CID_GAIN, 32);
    vivid.insert(CID_AUTO_WHITE_BALANCE, 1);
    vivid.insert(CID_EXPOSURE_AUTO, 3);
    vivid.insert(CID_POWER_LINE_FREQ, 2);
    presets.insert("Recommended (Vivid)", vivid);

    let mut daylight = HashMap::new();
    daylight.insert(CID_SATURATION, 50);
    daylight.insert(CID_CONTRAST, 40);
    daylight.insert(CID_BRIGHTNESS, 124);
    daylight.insert(CID_SHARPNESS, 26);
    daylight.insert(CID_GAIN, 24);
    daylight.insert(CID_WHITE_BALANCE_TEMP, 5200);
    daylight.insert(CID_AUTO_WHITE_BALANCE, 0);
    daylight.insert(CID_EXPOSURE_AUTO, 3);
    daylight.insert(CID_POWER_LINE_FREQ, 2);
    presets.insert("Daylight / Office", daylight);

    let mut warm = HashMap::new();
    warm.insert(CID_SATURATION, 54);
    warm.insert(CID_CONTRAST, 44);
    warm.insert(CID_BRIGHTNESS, 134);
    warm.insert(CID_SHARPNESS, 28);
    warm.insert(CID_GAIN, 36);
    warm.insert(CID_WHITE_BALANCE_TEMP, 3800);
    warm.insert(CID_AUTO_WHITE_BALANCE, 0);
    warm.insert(CID_EXPOSURE_AUTO, 3);
    warm.insert(CID_POWER_LINE_FREQ, 2);
    presets.insert("Night / Warm Lamp", warm);

    let mut cool = HashMap::new();
    cool.insert(CID_SATURATION, 58);
    cool.insert(CID_CONTRAST, 46);
    cool.insert(CID_BRIGHTNESS, 128);
    cool.insert(CID_SHARPNESS, 30);
    cool.insert(CID_GAIN, 32);
    cool.insert(CID_WHITE_BALANCE_TEMP, 6200);
    cool.insert(CID_AUTO_WHITE_BALANCE, 0);
    cool.insert(CID_EXPOSURE_AUTO, 3);
    cool.insert(CID_POWER_LINE_FREQ, 2);
    presets.insert("Cool / Studio", cool);

    presets
}

pub fn load_user_profiles() -> HashMap<String, HashMap<u32, i32>> {
    let path = get_profiles_file();
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(raw_map) = serde_json::from_str::<HashMap<String, HashMap<String, i32>>>(&data) {
            let mut result = HashMap::new();
            for (pname, vals) in raw_map {
                let mut int_vals = HashMap::new();
                for (cid_str, val) in vals {
                    if let Ok(cid) = cid_str.parse::<u32>() {
                        int_vals.insert(cid, val);
                    }
                }
                result.insert(pname, int_vals);
            }
            return result;
        }
    }
    HashMap::new()
}

pub fn save_user_profile(name: &str, values: &HashMap<u32, i32>) {
    let dir = get_config_dir();
    let _ = fs::create_dir_all(&dir);

    let mut profiles = load_user_profiles();
    profiles.insert(name.to_string(), values.clone());

    let mut raw_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for (pname, vals) in profiles {
        let mut str_vals = HashMap::new();
        for (cid, val) in vals {
            str_vals.insert(cid.to_string(), val);
        }
        raw_map.insert(pname, str_vals);
    }

    if let Ok(json_str) = serde_json::to_string_pretty(&raw_map) {
        let _ = fs::write(get_profiles_file(), json_str);
    }
}

pub fn delete_user_profile(name: &str) {
    let mut profiles = load_user_profiles();
    profiles.remove(name);

    let mut raw_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for (pname, vals) in profiles {
        let mut str_vals = HashMap::new();
        for (cid, val) in vals {
            str_vals.insert(cid.to_string(), val);
        }
        raw_map.insert(pname, str_vals);
    }

    if let Ok(json_str) = serde_json::to_string_pretty(&raw_map) {
        let _ = fs::write(get_profiles_file(), json_str);
    }
}

pub fn save_startup_default(dev_path: &str) -> Result<String, String> {
    let dir = get_config_dir();
    let _ = fs::create_dir_all(&dir);

    let cids = [
        CID_SATURATION, CID_CONTRAST, CID_BRIGHTNESS, CID_SHARPNESS,
        CID_GAIN, CID_WHITE_BALANCE_TEMP, CID_AUTO_WHITE_BALANCE,
        CID_EXPOSURE_AUTO, CID_EXPOSURE_ABSOLUTE, CID_POWER_LINE_FREQ
    ];

    let mut saved_state = HashMap::new();
    for &cid in &cids {
        if let Some(info) = query_control(dev_path, cid) {
            saved_state.insert(cid.to_string(), info.current);
        }
    }

    let json_str = serde_json::to_string_pretty(&saved_state).map_err(|e| e.to_string())?;
    fs::write(get_startup_file(), json_str).map_err(|e| e.to_string())?;

    // Create autostart entry
    let autostart_dir = get_autostart_file();
    if let Some(parent) = autostart_dir.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let autostart_content = "[Desktop Entry]\nType=Application\nName=Camera Settings Restore\nComment=Restores saved camera settings on login\nExec=cam-controls --restore\nTerminal=false\nX-GNOME-Autostart-enabled=true\nX-KDE-autostart-phase=2\n";
    let _ = fs::write(get_autostart_file(), autostart_content);

    Ok("Settings saved as startup default. Autostart enabled!".to_string())
}

pub fn restore_startup_defaults() {
    let path = get_startup_file();
    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(saved_state) = serde_json::from_str::<HashMap<String, i32>>(&data) {
            let cameras = list_capture_cameras();
            for cam in cameras {
                for (cid_str, val) in &saved_state {
                    if let Ok(cid) = cid_str.parse::<u32>() {
                        set_control(&cam.path, cid, *val);
                    }
                }
            }
        }
    }
}

pub fn fix_pipewire() -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let conf_dir = PathBuf::from(home).join(".config").join("wireplumber").join("wireplumber.conf.d");
    let _ = fs::create_dir_all(&conf_dir);
    let target = conf_dir.join("99-disable-libcamera.conf");

    let content = "# Generated by Camera Quick Controls (Rust Edition)\n# Bypasses experimental libcamera UVC handler\nwireplumber.profiles = {\n  main = {\n    monitor.libcamera = \"disabled\"\n  }\n}\n";
    fs::write(&target, content).map_err(|e| e.to_string())?;

    let _ = Command::new("systemctl")
        .args(["--user", "restart", "wireplumber", "pipewire"])
        .status();

    Ok("WirePlumber configuration updated to bypass libcamera.\nPipeWire restarted!".to_string())
}

pub fn fix_microphone() -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let conf_dir = PathBuf::from(home).join(".config").join("wireplumber").join("wireplumber.conf.d");
    let _ = fs::create_dir_all(&conf_dir);
    let target = conf_dir.join("50-c310-mic.conf");

    let content = "# Generated by Camera Quick Controls (Rust Edition)\n# Fixes Logitech C310 16kHz microphone bug\nmonitor.alsa.rules = [\n  {\n    matches = [\n      {\n        node.name = \"~alsa_input.usb-.*046d.*\"\n      }\n    ]\n    actions = {\n      update-props = {\n        audio.rate = 16000\n        audio.allowed-rates = [ 16000 ]\n      }\n    }\n  }\n]\n";
    fs::write(&target, content).map_err(|e| e.to_string())?;

    let _ = Command::new("systemctl")
        .args(["--user", "restart", "wireplumber", "pipewire"])
        .status();

    Ok("Microphone sample rate locked to 16000 Hz.\nPipeWire restarted!".to_string())
}
