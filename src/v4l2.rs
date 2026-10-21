use std::collections::HashMap;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub const VIDIOC_QUERYCAP: u64  = 0x80685600;
pub const VIDIOC_QUERYCTRL: u64 = 0xc0445624;
pub const VIDIOC_G_CTRL: u64    = 0xc008561b;
pub const VIDIOC_S_CTRL: u64    = 0xc008561c;

pub const V4L2_CAP_VIDEO_CAPTURE: u32 = 0x00000001;
pub const V4L2_CAP_DEVICE_CAPS: u32   = 0x80000000;

pub const CID_BRIGHTNESS: u32         = 0x00980900;
pub const CID_CONTRAST: u32           = 0x00980901;
pub const CID_SATURATION: u32         = 0x00980902;
pub const CID_AUTO_WHITE_BALANCE: u32 = 0x0098090c;
pub const CID_GAIN: u32               = 0x00980913;
pub const CID_POWER_LINE_FREQ: u32    = 0x00980918;
pub const CID_WHITE_BALANCE_TEMP: u32 = 0x0098091a;
pub const CID_SHARPNESS: u32          = 0x0098091b;
pub const CID_BACKLIGHT_COMP: u32     = 0x0098091c;
pub const CID_EXPOSURE_AUTO: u32      = 0x009a0901;
pub const CID_EXPOSURE_ABSOLUTE: u32  = 0x009a0902;

extern "C" {
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

#[repr(C)]
pub struct V4l2Capability {
    pub driver: [u8; 16],
    pub card: [u8; 32],
    pub bus_info: [u8; 32],
    pub version: u32,
    pub capabilities: u32,
    pub device_caps: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
pub struct V4l2QueryCtrl {
    pub id: u32,
    pub ctrl_type: u32,
    pub name: [u8; 32],
    pub minimum: i32,
    pub maximum: i32,
    pub step: i32,
    pub default_value: i32,
    pub flags: u32,
    pub reserved: [u32; 2],
}

#[repr(C)]
pub struct V4l2Control {
    pub id: u32,
    pub value: i32,
}

#[derive(Clone, Debug)]
pub struct CameraDevice {
    pub path: String,
    pub name: String,
    pub driver: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ControlInfo {
    pub cid: u32,
    pub name: String,
    pub ctrl_type: u32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub default: i32,
    pub current: i32,
}

pub fn list_capture_cameras() -> Vec<CameraDevice> {
    let mut cameras = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    // 1. Check stable /dev/v4l/by-id/ index0 symlinks first
    if let Ok(entries) = std::fs::read_dir("/dev/v4l/by-id") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                if fname.contains("index0") {
                    if let Ok(target) = std::fs::canonicalize(&path) {
                        let target_str = target.to_string_lossy().to_string();
                        if let Some(cam) = probe_camera(&target_str) {
                            seen_paths.insert(target_str);
                            cameras.push(cam);
                        }
                    }
                }
            }
        }
    }

    // 2. Check standard /dev/video* devices
    if let Ok(entries) = std::fs::read_dir("/dev") {
        let mut dev_paths: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .and_then(|f| f.to_str())
                    .map_or(false, |s| s.starts_with("video") && s[5..].chars().all(|c| c.is_ascii_digit()))
            })
            .collect();
        dev_paths.sort();

        for path in dev_paths {
            let path_str = path.to_string_lossy().to_string();
            if seen_paths.contains(&path_str) {
                continue;
            }
            if let Some(cam) = probe_camera(&path_str) {
                seen_paths.insert(path_str);
                cameras.push(cam);
            }
        }
    }

    cameras
}

pub fn probe_camera(dev_path: &str) -> Option<CameraDevice> {
    let file = OpenOptions::new().read(true).write(true).open(dev_path).ok()?;
    let fd = file.as_raw_fd();

    let mut cap: V4l2Capability = unsafe { std::mem::zeroed() };
    let res = unsafe { ioctl(fd, VIDIOC_QUERYCAP, &mut cap) };
    if res != 0 {
        return None;
    }

    let eff_caps = if (cap.capabilities & V4L2_CAP_DEVICE_CAPS) != 0 {
        cap.device_caps
    } else {
        cap.capabilities
    };

    if (eff_caps & V4L2_CAP_VIDEO_CAPTURE) == 0 {
        return None;
    }

    let card = String::from_utf8_lossy(&cap.card);
    let card = card.trim_matches('\0').trim().to_string();
    let driver = String::from_utf8_lossy(&cap.driver);
    let driver = driver.trim_matches('\0').trim().to_string();

    Some(CameraDevice {
        path: dev_path.to_string(),
        name: if card.is_empty() { format!("Camera ({})", dev_path) } else { card },
        driver,
    })
}

pub fn query_control(dev_path: &str, cid: u32) -> Option<ControlInfo> {
    let file = OpenOptions::new().read(true).write(true).open(dev_path).ok()?;
    let fd = file.as_raw_fd();

    let mut q: V4l2QueryCtrl = unsafe { std::mem::zeroed() };
    q.id = cid;

    let res = unsafe { ioctl(fd, VIDIOC_QUERYCTRL, &mut q) };
    if res != 0 || (q.flags & 1) != 0 {
        return None;
    }

    let mut ctrl = V4l2Control { id: cid, value: 0 };
    let res_ctrl = unsafe { ioctl(fd, VIDIOC_G_CTRL, &mut ctrl) };
    let current = if res_ctrl == 0 { ctrl.value } else { q.default_value };

    let name = String::from_utf8_lossy(&q.name);
    let name = name.trim_matches('\0').trim().to_string();

    Some(ControlInfo {
        cid,
        name,
        ctrl_type: q.ctrl_type,
        min: q.minimum,
        max: q.maximum,
        step: q.step,
        default: q.default_value,
        current,
    })
}

pub fn set_control(dev_path: &str, cid: u32, val: i32) -> bool {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(dev_path) {
        let fd = file.as_raw_fd();
        let mut ctrl = V4l2Control { id: cid, value: val };
        let res = unsafe { ioctl(fd, VIDIOC_S_CTRL, &mut ctrl) };
        res == 0
    } else {
        false
    }
}

pub fn query_all_controls(dev_path: &str) -> Vec<ControlInfo> {
    let mut controls = Vec::new();
    let cids = [
        CID_SATURATION,
        CID_CONTRAST,
        CID_BRIGHTNESS,
        CID_SHARPNESS,
        CID_WHITE_BALANCE_TEMP,
        CID_AUTO_WHITE_BALANCE,
        CID_GAIN,
        CID_EXPOSURE_AUTO,
        CID_EXPOSURE_ABSOLUTE,
        CID_POWER_LINE_FREQ,
        CID_BACKLIGHT_COMP,
    ];

    for &cid in &cids {
        if let Some(info) = query_control(dev_path, cid) {
            controls.push(info);
        }
    }
    controls
}

#[derive(Clone)]
pub struct AsyncWorker {
    state: Arc<(Mutex<HashMap<(String, u32), i32>>, Condvar)>,
}

impl AsyncWorker {
    pub fn new() -> Self {
        let state = Arc::new((Mutex::new(HashMap::new()), Condvar::new()));
        let worker_state = Arc::clone(&state);

        thread::spawn(move || {
            let (lock, cvar) = &*worker_state;
            loop {
                let pending: Vec<((String, u32), i32)> = {
                    let mut queue = lock.lock().unwrap();
                    while queue.is_empty() {
                        queue = cvar.wait(queue).unwrap();
                    }
                    queue.drain().collect()
                };

                for ((dev, cid), val) in pending {
                    set_control(&dev, cid, val);
                    thread::sleep(std::time::Duration::from_millis(8));
                }
            }
        });

        Self { state }
    }

    pub fn set_control_async(&self, dev_path: &str, cid: u32, val: i32) {
        let (lock, cvar) = &*self.state;
        if let Ok(mut queue) = lock.lock() {
            queue.insert((dev_path.to_string(), cid), val);
            cvar.notify_one();
        }
    }
}
