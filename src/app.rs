use eframe::egui;
use std::sync::{Arc, Mutex};
use eframe::egui::{FontId, RichText, TextEdit};
use crate::pcileech;

struct CommandDefinition {
    label: &'static str,
    command: &'static str,
}

const QUICK_COMMANDS: &[CommandDefinition] = &[
    CommandDefinition {
        label: "Probe Device",
        command: "probe"
    },
    CommandDefinition {
        label: "Inject \"5x shift = CMD\"",
        command: "patch -sig stickykeys_cmd_win.sig -all -psname winlogon.exe",
    },
];

const OTHER_COMMANDS: &[CommandDefinition] = &[
    CommandDefinition { label: "Display (0-1000)", command: "display -min 0x0 -max 0x1000" },
    CommandDefinition { label: "Display (1000)", command: "display -min 0x1000" },
    CommandDefinition { label: "KMD Load", command: "kmdload -kmd win7_x64" },
    CommandDefinition { label: "Dump Memory", command: "dump --out mem_dump.raw" },
    CommandDefinition { label: "Help", command: "help" },
];

pub struct PciLeechApp {
    pub command_args: String,
    pub output: Arc<Mutex<String>>,
    pub is_running: Arc<Mutex<bool>>,
}

impl Default for PciLeechApp {
    fn default() -> Self {
        Self {
            command_args: "display -min 0x0 -max 0x1000".to_owned(),
            output: Arc::new(Mutex::new(String::new())),
            is_running: Arc::new(Mutex::new(false)),
        }
    }
}

impl PciLeechApp {
    fn run_pcileech(&self) {
        pcileech::run_pcileech(
            self.command_args.clone(),
            self.output.clone(),
            self.is_running.clone(),
        );
    }
}

impl eframe::App for PciLeechApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {

            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("PCILeechGui")
                        .size(40.0)
                );
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Arguments:")
                            .size(15.0)
                    );
                    ui.add(
                        TextEdit::singleline(&mut self.command_args)
                            .font(FontId::proportional(15.0))
                    );

                    if ui.button(RichText::new("Run command").size(15.0)).clicked() {
                        self.run_pcileech();
                    }
                });

                ui.add_space(10.0);

                ui.label("Quick Commands:");
                let is_running = *self.is_running.lock().unwrap();
                ui.add_enabled_ui(!is_running, |ui| {
                    ui.horizontal(|ui| {
                        for cmd in QUICK_COMMANDS {
                            if ui.button(cmd.label).clicked() {
                                self.command_args = cmd.command.to_owned();
                                self.run_pcileech();
                            }
                        }
                    });

                    ui.label("Other Commands:");
                    ui.horizontal_wrapped(|ui| {
                        for cmd in OTHER_COMMANDS {
                            if ui.button(cmd.label).clicked() {
                                self.command_args = cmd.command.to_owned();
                            }
                        }

                    });
                });

                if is_running {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Running...");
                    });
                }
            });
        });

        egui::Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear Output").clicked() {
                    self.output.lock().unwrap().clear();
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Output:");
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let mut output = self.output.lock().unwrap();
                    ui.add(
                        egui::TextEdit::multiline(&mut *output)
                            .interactive(false)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                });
        });

        if *self.is_running.lock().unwrap() {
            ui.ctx().request_repaint();
        }
    }
}
