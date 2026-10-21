mod v4l2;
mod config;
mod gui;

use std::env;
use std::process::exit;
use crate::v4l2::*;
use crate::config::*;

const VERSION: &str = "2.0.0";

fn print_help() {
    println!("cam-controls {} - High-performance camera image tuning & PipeWire fix for Linux", VERSION);
    println!("\nUsage: cam-controls [OPTIONS]");
    println!("\nOptions:");
    println!("  -l, --list-devices        List all detected camera devices");
    println!("  -d, --device <DEV>        Target camera device path (e.g. /dev/video0) or index");
    println!("      --status              Print current camera control values");
    println!("      --apply-recommended   Apply recommended vivid image settings");
    println!("      --reset-defaults      Reset camera controls to hardware defaults");
    println!("      --save-default        Save current settings as startup default (enables login autostart)");
    println!("      --restore             Silently restore saved startup defaults (used on login)");
    println!("      --fix-pipewire        Fix garbled/green video by bypassing libcamera");
    println!("      --fix-mic             Fix robotic/fast microphone audio by locking 16kHz");
    println!("      --gui                 Launch GUI control panel (default if desktop display detected)");
    println!("  -h, --help                Show this help message");
    println!("  -v, --version             Print version information");
}

fn print_status(dev_path: &str) {
    let canonical = std::fs::canonicalize(dev_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| dev_path.to_string());
    println!("Camera Device: {} ({})", dev_path, canonical);
    println!("{}", "-".repeat(65));
    let cids = [
        CID_SATURATION, CID_CONTRAST, CID_BRIGHTNESS, CID_SHARPNESS,
        CID_WHITE_BALANCE_TEMP, CID_AUTO_WHITE_BALANCE, CID_GAIN,
        CID_EXPOSURE_AUTO, CID_EXPOSURE_ABSOLUTE, CID_POWER_LINE_FREQ
    ];
    for &cid in &cids {
        if let Some(info) = query_control(dev_path, cid) {
            println!("{:25}: Current={:<5} (Min={}, Max={}, Default={})", info.name, info.current, info.min, info.max, info.default);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut target_device: Option<String> = None;
    let mut do_list = false;
    let mut do_status = false;
    let mut do_apply_recommended = false;
    let mut do_reset = false;
    let mut do_save_default = false;
    let mut do_restore = false;
    let mut do_fix_pipewire = false;
    let mut do_fix_mic = false;
    let mut force_gui = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                println!("cam-controls {}", VERSION);
                return;
            }
            "-l" | "--list-devices" => {
                do_list = true;
            }
            "--status" => {
                do_status = true;
            }
            "--apply-recommended" => {
                do_apply_recommended = true;
            }
            "--reset-defaults" => {
                do_reset = true;
            }
            "--save-default" => {
                do_save_default = true;
            }
            "--restore" => {
                do_restore = true;
            }
            "--fix-pipewire" => {
                do_fix_pipewire = true;
            }
            "--fix-mic" => {
                do_fix_mic = true;
            }
            "--gui" => {
                force_gui = true;
            }
            "-d" | "--device" => {
                if i + 1 < args.len() {
                    target_device = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: -d / --device requires a camera path or index.");
                    exit(1);
                }
            }
            other => {
                eprintln!("Error: Unknown option '{}'. Run 'cam-controls --help' for usage.", other);
                exit(1);
            }
        }
        i += 1;
    }

    // 1. Headless standalone actions
    if do_restore {
        restore_startup_defaults();
        return;
    }

    if do_fix_pipewire {
        match fix_pipewire() {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("Error fixing PipeWire: {}", err),
        }
        return;
    }

    if do_fix_mic {
        match fix_microphone() {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("Error fixing microphone: {}", err),
        }
        return;
    }

    let cameras = list_capture_cameras();

    if do_list {
        if cameras.is_empty() {
            println!("No video capture devices found.");
        } else {
            println!("Found {} video capture camera(s):", cameras.len());
            for (idx, cam) in cameras.iter().enumerate() {
                println!("  [{}] {:30} -> {} ({})", idx, cam.name, cam.path, cam.driver);
            }
        }
        return;
    }

    // Resolve target camera device
    let dev_path = match target_device {
        Some(ref d) if d.chars().all(|c| c.is_ascii_digit()) => {
            let idx: usize = d.parse().unwrap_or(0);
            if idx < cameras.len() {
                cameras[idx].path.clone()
            } else {
                eprintln!("Error: Camera index {} out of range (found {} cameras).", idx, cameras.len());
                exit(1);
            }
        }
        Some(ref d) => {
            if std::path::Path::new(d).exists() {
                d.clone()
            } else {
                eprintln!("Error: Specified device '{}' does not exist.", d);
                exit(1);
            }
        }
        None => {
            if let Some(first) = cameras.first() {
                first.path.clone()
            } else {
                "/dev/video0".to_string()
            }
        }
    };

    if do_save_default {
        match save_startup_default(&dev_path) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("Error: {}", err),
        }
        return;
    }

    if do_status {
        print_status(&dev_path);
        return;
    }

    if do_apply_recommended {
        let presets = get_builtin_presets();
        if let Some(vivid) = presets.get("Recommended (Vivid)") {
            for (&cid, &val) in vivid {
                set_control(&dev_path, cid, val);
            }
            println!("Recommended vivid profile applied to {}!", dev_path);
        }
        return;
    }

    if do_reset {
        for cid in [CID_SATURATION, CID_CONTRAST, CID_BRIGHTNESS, CID_SHARPNESS, CID_GAIN] {
            if let Some(info) = query_control(&dev_path, cid) {
                set_control(&dev_path, cid, info.default);
            }
        }
        println!("Hardware defaults restored on {}!", dev_path);
        return;
    }

    // If GUI forced or desktop display detected, launch GUI
    let has_display = env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok();
    if force_gui || (has_display && args.len() == 1) {
        if let Err(e) = gui::run_gui(Some(dev_path)) {
            eprintln!("Failed to launch GUI: {}", e);
            exit(1);
        }
    } else {
        print_status(&dev_path);
    }
}
