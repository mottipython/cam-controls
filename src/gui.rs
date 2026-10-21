use eframe::egui;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::v4l2::*;
use crate::config::*;

pub struct CamApp {
    cameras: Vec<CameraDevice>,
    selected_camera_idx: usize,
    worker: AsyncWorker,

    // Controls state for selected camera: cid -> ControlInfo
    controls: HashMap<u32, ControlInfo>,
    slider_values: HashMap<u32, i32>,

    // Smart toggles
    auto_wb: bool,
    auto_exp: bool,
    anti_flicker: usize,

    // Profiles
    builtin_presets: HashMap<&'static str, HashMap<u32, i32>>,
    user_profiles: HashMap<String, HashMap<u32, i32>>,
    selected_preset: String,

    // Save profile modal
    show_save_dialog: bool,
    new_profile_name: String,

    // Notification toast: (message, created_at, is_error)
    notification: Option<(String, Instant, bool)>,
}

impl CamApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, initial_dev: Option<String>) -> Self {
        let cameras = list_capture_cameras();
        let worker = AsyncWorker::new();
        let builtin_presets = get_builtin_presets();
        let user_profiles = load_user_profiles();

        let mut selected_camera_idx = 0;
        if let Some(target) = initial_dev {
            for (i, cam) in cameras.iter().enumerate() {
                if cam.path == target {
                    selected_camera_idx = i;
                    break;
                }
            }
        }

        let mut app = Self {
            cameras,
            selected_camera_idx,
            worker,
            controls: HashMap::new(),
            slider_values: HashMap::new(),
            auto_wb: true,
            auto_exp: true,
            anti_flicker: 2,
            builtin_presets,
            user_profiles,
            selected_preset: "Recommended (Vivid)".to_string(),
            show_save_dialog: false,
            new_profile_name: String::new(),
            notification: None,
        };

        if !app.cameras.is_empty() {
            app.load_camera(selected_camera_idx);
        }

        app
    }

    pub fn reload_cameras(&mut self) {
        self.cameras = list_capture_cameras();
        if self.cameras.is_empty() {
            self.controls.clear();
            self.slider_values.clear();
            self.selected_camera_idx = 0;
            self.notification = Some(("No video capture cameras detected.".to_string(), Instant::now(), true));
        } else {
            if self.selected_camera_idx >= self.cameras.len() {
                self.selected_camera_idx = 0;
            }
            self.load_camera(self.selected_camera_idx);
        }
    }

    pub fn load_camera(&mut self, idx: usize) {
        if idx >= self.cameras.len() {
            return;
        }
        self.selected_camera_idx = idx;
        let dev = &self.cameras[idx].path;

        let ctrl_list = query_all_controls(dev);
        self.controls.clear();
        self.slider_values.clear();

        for ctrl in ctrl_list {
            self.slider_values.insert(ctrl.cid, ctrl.current);
            self.controls.insert(ctrl.cid, ctrl);
        }

        if let Some(info) = self.controls.get(&CID_AUTO_WHITE_BALANCE) {
            self.auto_wb = info.current != 0;
        }
        if let Some(info) = self.controls.get(&CID_EXPOSURE_AUTO) {
            self.auto_exp = info.current == 3;
        }
        if let Some(info) = self.controls.get(&CID_POWER_LINE_FREQ) {
            self.anti_flicker = (info.current.clamp(0, 2)) as usize;
        }
    }

    pub fn current_dev_path(&self) -> Option<&str> {
        self.cameras.get(self.selected_camera_idx).map(|c| c.path.as_str())
    }

    pub fn apply_preset_map(&mut self, values: &HashMap<u32, i32>) {
        if let Some(dev) = self.current_dev_path() {
            let dev_str = dev.to_string();
            for (&cid, &val) in values {
                self.worker.set_control_async(&dev_str, cid, val);
                self.slider_values.insert(cid, val);

                if cid == CID_AUTO_WHITE_BALANCE {
                    self.auto_wb = val != 0;
                } else if cid == CID_EXPOSURE_AUTO {
                    self.auto_exp = val == 3;
                } else if cid == CID_POWER_LINE_FREQ {
                    self.anti_flicker = (val.clamp(0, 2)) as usize;
                }
            }
        }
    }

    pub fn apply_selected_preset(&mut self) {
        let name = self.selected_preset.clone();
        if let Some(preset) = self.builtin_presets.get(name.as_str()) {
            let map = preset.clone();
            self.apply_preset_map(&map);
            self.set_notification(format!("Applied preset '{}'", name), false);
        } else if let Some(clean_name) = name.strip_prefix("★ ") {
            if let Some(profile) = self.user_profiles.get(clean_name) {
                let map = profile.clone();
                self.apply_preset_map(&map);
                self.set_notification(format!("Applied profile '{}'", clean_name), false);
            }
        }
    }

    pub fn set_notification(&mut self, msg: String, is_error: bool) {
        self.notification = Some((msg, Instant::now(), is_error));
    }
}

impl eframe::App for CamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clear notifications after 5 seconds
        if let Some((_, created, _)) = &self.notification {
            if created.elapsed() > Duration::from_secs(5) {
                self.notification = None;
            }
        }

        // Top-level CentralPanel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);

            // ==================== 1. Camera Selector ====================
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Camera:").strong());

                let current_label = if self.cameras.is_empty() {
                    "No Camera Found".to_string()
                } else {
                    format!("{} ({})", self.cameras[self.selected_camera_idx].name, self.cameras[self.selected_camera_idx].path)
                };

                let combo_width = ui.available_width() - 36.0;
                let mut switch_cam_idx = None;
                egui::ComboBox::from_id_source("cam_selector")
                    .width(combo_width)
                    .selected_text(current_label)
                    .show_ui(ui, |ui| {
                        for (i, cam) in self.cameras.iter().enumerate() {
                            let label = format!("{} ({})", cam.name, cam.path);
                            if ui.selectable_value(&mut self.selected_camera_idx, i, label).clicked() {
                                switch_cam_idx = Some(i);
                            }
                        }
                    });

                if let Some(idx) = switch_cam_idx {
                    self.load_camera(idx);
                }

                if ui.button("↻").on_hover_text("Refresh connected camera list").clicked() {
                    self.reload_cameras();
                }
            });

            if self.cameras.is_empty() {
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.colored_label(egui::Color32::from_rgb(220, 50, 50), "⚠️ No video capture cameras detected.");
                    ui.label("Please connect your USB webcam and click ↻ refresh above.");
                });
                return;
            }

            // ==================== 2. Preset Selector ====================
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Profile:").strong());

                let combo_width = ui.available_width() - 130.0;
                egui::ComboBox::from_id_source("preset_selector")
                    .width(combo_width)
                    .selected_text(&self.selected_preset)
                    .show_ui(ui, |ui| {
                        let builtins = ["Recommended (Vivid)", "Daylight / Office", "Night / Warm Lamp", "Cool / Studio"];
                        for name in builtins {
                            if ui.selectable_value(&mut self.selected_preset, name.to_string(), name).clicked() {
                                self.apply_selected_preset();
                            }
                        }

                        if !self.user_profiles.is_empty() {
                            ui.separator();
                            let mut profile_names: Vec<String> = self.user_profiles.keys().cloned().collect();
                            profile_names.sort();
                            for name in profile_names {
                                let display_name = format!("★ {}", name);
                                if ui.selectable_value(&mut self.selected_preset, display_name.clone(), display_name).clicked() {
                                    self.apply_selected_preset();
                                }
                            }
                        }
                    });

                if ui.button("Save As...").on_hover_text("Save current settings as a new custom preset").clicked() {
                    self.new_profile_name.clear();
                    self.show_save_dialog = true;
                }

                let is_custom = self.selected_preset.starts_with("★ ");
                if ui.add_enabled(is_custom, egui::Button::new("Del")).on_hover_text("Delete selected custom profile").clicked() {
                    if let Some(clean_name) = self.selected_preset.strip_prefix("★ ").map(|s| s.to_string()) {
                        delete_user_profile(&clean_name);
                        self.user_profiles = load_user_profiles();
                        self.selected_preset = "Recommended (Vivid)".to_string();
                        self.set_notification(format!("Deleted profile '{}'", clean_name), false);
                    }
                }
            });

            ui.add_space(2.0);

            // ==================== 3. Smart Adjustments Frame ====================
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new("Smart Adjustments & Lighting").strong());
                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    let has_awb = self.controls.contains_key(&CID_AUTO_WHITE_BALANCE);
                    let has_aexp = self.controls.contains_key(&CID_EXPOSURE_AUTO);

                    ui.vertical(|ui| {
                        if ui.add_enabled(has_awb, egui::Checkbox::new(&mut self.auto_wb, "Auto White Balance")).changed() {
                            if let Some(dev) = self.current_dev_path() {
                                self.worker.set_control_async(dev, CID_AUTO_WHITE_BALANCE, if self.auto_wb { 1 } else { 0 });
                            }
                        }
                        if ui.add_enabled(has_aexp, egui::Checkbox::new(&mut self.auto_exp, "Auto Exposure")).changed() {
                            if let Some(dev) = self.current_dev_path() {
                                self.worker.set_control_async(dev, CID_EXPOSURE_AUTO, if self.auto_exp { 3 } else { 1 });
                            }
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let has_flicker = self.controls.contains_key(&CID_POWER_LINE_FREQ);
                        let flicker_labels = ["Disabled", "50 Hz (EU / Asia)", "60 Hz (Americas)"];
                        let current_text = flicker_labels.get(self.anti_flicker).unwrap_or(&"Disabled");

                        ui.add_enabled_ui(has_flicker, |ui| {
                            egui::ComboBox::from_id_source("flicker_combo")
                                .width(140.0)
                                .selected_text(*current_text)
                                .show_ui(ui, |ui| {
                                    for (idx, text) in flicker_labels.iter().enumerate() {
                                        if ui.selectable_value(&mut self.anti_flicker, idx, *text).clicked() {
                                            if let Some(dev) = self.current_dev_path() {
                                                self.worker.set_control_async(dev, CID_POWER_LINE_FREQ, idx as i32);
                                            }
                                        }
                                    }
                                });
                            ui.label("Anti-Flicker:");
                        });
                    });
                });
            });

            ui.add_space(2.0);

            // ==================== 4. Manual Sliders Frame ====================
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new("Manual Image Controls").strong());
                ui.add_space(4.0);

                let slider_cids = [
                    (CID_SATURATION, "Saturation"),
                    (CID_CONTRAST, "Contrast"),
                    (CID_BRIGHTNESS, "Brightness"),
                    (CID_SHARPNESS, "Sharpness"),
                    (CID_WHITE_BALANCE_TEMP, "White Balance"),
                    (CID_EXPOSURE_ABSOLUTE, "Exposure Time"),
                    (CID_GAIN, "Gain"),
                ];

                for (cid, label) in slider_cids {
                    if let Some(info) = self.controls.get(&cid).cloned() {
                        let is_enabled = match cid {
                            CID_WHITE_BALANCE_TEMP => !self.auto_wb,
                            CID_EXPOSURE_ABSOLUTE => !self.auto_exp,
                            _ => true,
                        };

                        let mut val = *self.slider_values.entry(cid).or_insert(info.current);
                        let mut changed = false;

                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(is_enabled, |ui| {
                                ui.label(format!("{}:", label));
                            });

                            let val_str = format!("{}", val);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new(val_str).monospace());

                                let slider_width = ui.available_width() - 8.0;
                                let slider = egui::Slider::new(&mut val, info.min..=info.max)
                                    .show_value(false)
                                    .clamp_to_range(true);

                                let resp = ui.add_sized([slider_width, 18.0], slider);
                                if resp.changed() {
                                    changed = true;
                                }
                            });
                        });

                        if changed {
                            self.slider_values.insert(cid, val);
                            if let Some(dev) = self.current_dev_path() {
                                self.worker.set_control_async(dev, cid, val);
                            }
                        }
                    }
                }
            });

            ui.add_space(2.0);

            // ==================== 5. Default Action Buttons ====================
            ui.horizontal(|ui| {
                if ui.button("Reset Defaults").on_hover_text("Reset all controls to hardware defaults").clicked() {
                    if let Some(dev) = self.current_dev_path() {
                        let dev_str = dev.to_string();
                        for (cid, info) in &self.controls {
                            self.slider_values.insert(*cid, info.default);
                            self.worker.set_control_async(&dev_str, *cid, info.default);
                        }
                        if let Some(info) = self.controls.get(&CID_AUTO_WHITE_BALANCE) {
                            self.auto_wb = info.default != 0;
                        }
                        if let Some(info) = self.controls.get(&CID_EXPOSURE_AUTO) {
                            self.auto_exp = info.default == 3;
                        }
                        if let Some(info) = self.controls.get(&CID_POWER_LINE_FREQ) {
                            self.anti_flicker = (info.default.clamp(0, 2)) as usize;
                        }
                        self.set_notification("Hardware defaults restored!".to_string(), false);
                    }
                }

                if ui.button("★ Save as Default").on_hover_text("Save current settings as startup default on login").clicked() {
                    if let Some(dev) = self.current_dev_path() {
                        match save_startup_default(dev) {
                            Ok(msg) => self.set_notification(msg, false),
                            Err(err) => self.set_notification(format!("Error: {}", err), true),
                        }
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Apply Recommended").on_hover_text("Apply vivid, high-clarity recommended settings").clicked() {
                        if let Some(preset) = self.builtin_presets.get("Recommended (Vivid)") {
                            let map = preset.clone();
                            self.apply_preset_map(&map);
                            self.selected_preset = "Recommended (Vivid)".to_string();
                            self.set_notification("Applied Recommended (Vivid) preset!".to_string(), false);
                        }
                    }
                });
            });

            ui.add_space(2.0);

            // ==================== 6. One-Click Linux Fixes Frame ====================
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new("One-Click System Fixes").strong());
                ui.add_space(2.0);

                if ui.button("🛡️ Fix Garbled / Green Video (PipeWire)").on_hover_text("Configures WirePlumber to bypass experimental libcamera UVC handler").clicked() {
                    match fix_pipewire() {
                        Ok(msg) => self.set_notification(msg, false),
                        Err(err) => self.set_notification(format!("Error: {}", err), true),
                    }
                }

                if ui.button("🎙️ Fix Robotic / Fast Microphone Audio (16kHz)").on_hover_text("Locks Logitech C310 / UVC microphone sample rate to 16000 Hz in WirePlumber").clicked() {
                    match fix_microphone() {
                        Ok(msg) => self.set_notification(msg, false),
                        Err(err) => self.set_notification(format!("Error: {}", err), true),
                    }
                }
            });

            // ==================== 7. Notification Banner ====================
            if let Some((msg, _, is_error)) = &self.notification {
                ui.add_space(4.0);
                let color = if *is_error {
                    egui::Color32::from_rgb(220, 60, 60)
                } else {
                    egui::Color32::from_rgb(46, 160, 67)
                };
                egui::Frame::none()
                    .fill(color.gamma_multiply(0.2))
                    .stroke(egui::Stroke::new(1.0_f32, color))
                    .inner_margin(8.0)
                    .rounding(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.colored_label(color, msg);
                        });
                    });
            }
        });

        // Save Profile Modal Dialog
        if self.show_save_dialog {
            egui::Window::new("Save Custom Profile")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("Enter a name for this custom profile:");
                    let resp = ui.text_edit_singleline(&mut self.new_profile_name);
                    resp.request_focus();

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() || (ui.input(|i| i.key_pressed(egui::Key::Enter)) && !self.new_profile_name.trim().is_empty()) {
                            let name = self.new_profile_name.trim().to_string();
                            if !name.is_empty() {
                                let mut vals = self.slider_values.clone();
                                vals.insert(CID_AUTO_WHITE_BALANCE, if self.auto_wb { 1 } else { 0 });
                                vals.insert(CID_EXPOSURE_AUTO, if self.auto_exp { 3 } else { 1 });
                                vals.insert(CID_POWER_LINE_FREQ, self.anti_flicker as i32);

                                save_user_profile(&name, &vals);
                                self.user_profiles = load_user_profiles();
                                self.selected_preset = format!("★ {}", name);
                                self.set_notification(format!("Saved profile '{}'!", name), false);
                                self.show_save_dialog = false;
                            }
                        }

                        if ui.button("Cancel").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                            self.show_save_dialog = false;
                        }
                    });
                });
        }
    }
}

fn load_app_icon() -> Option<egui::IconData> {
    let icon_bytes = include_bytes!("../data/icons/hicolor/48x48/apps/cam-controls.png");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Some(egui::IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        })
    } else {
        None
    }
}

pub fn run_gui(initial_dev: Option<String>) -> Result<(), eframe::Error> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Camera Settings & Quick Controls")
        .with_inner_size([520.0, 680.0])
        .with_min_inner_size([460.0, 560.0])
        .with_app_id("cam-controls");

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Camera Settings & Quick Controls",
        native_options,
        Box::new(|cc| Ok(Box::new(CamApp::new(cc, initial_dev)))),
    )
}
