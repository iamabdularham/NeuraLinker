mod terminal;

use eframe::egui;
use egui_plotter::{Chart, MouseConfig};
use sysinfo::{System, Networks};
use crate::terminal::terminal::EmbeddedTerminal;
use std::time::{Duration, Instant};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "NeuraLinker",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

enum View {
    CPU,
    Memory,
    Network,
    Terminal,
}

struct MyApp {
    system: System,
    networks: Networks,
    terminal: EmbeddedTerminal,
    terminal_input: String,
    last_update: Instant,
    current_view: View,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            system: System::new_all(),
            networks: Networks::new_with_refreshed_list(),
            terminal: EmbeddedTerminal::new(),
            terminal_input: String::new(),
            last_update: Instant::now(),
            current_view: View::CPU,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Refresh system stats every 2 seconds
        if self.last_update.elapsed() > Duration::from_secs(2) {
            self.system.refresh_all();
            self.networks.refresh(true);
            self.last_update = Instant::now();
        }

        // Sidebar
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading("NeuraLinker");
                    ui.separator();

                    let button_size = egui::vec2(150.0, 50.0);

                    if ui.add_sized(button_size, egui::Button::new("CPU").corner_radius(25.0)).clicked() {
                        self.current_view = View::CPU;
                    }
                    ui.add_space(10.0);
                    if ui.add_sized(button_size, egui::Button::new("Memory").corner_radius(25.0)).clicked() {
                        self.current_view = View::Memory;
                    }
                    ui.add_space(10.0);
                    if ui.add_sized(button_size, egui::Button::new("Network").corner_radius(25.0)).clicked() {
                        self.current_view = View::Network;
                    }
                    ui.add_space(10.0);
                    if ui.add_sized(button_size, egui::Button::new("Terminal").corner_radius(25.0)).clicked() {
                        self.current_view = View::Terminal;
                    }
                });
            });

        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            match self.current_view {
                View::CPU => {
                    ui.heading("CPU Usage");
                    ui.separator();
                    for (i, cpu) in self.system.cpus().iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Core {}:", i + 1));
                            ui.add(egui::ProgressBar::new(cpu.cpu_usage() / 100.0).show_percentage());
                        });
                    }
                }
                View::Memory => {
                    ui.heading("Memory Usage");
                    ui.separator();
                    let total_memory = self.system.total_memory() / 1024;
                    let used_memory = self.system.used_memory() / 1024;
                    ui.label(format!("Used: {} MB", used_memory));
                    ui.label(format!("Total: {} MB", total_memory));
                    ui.add(egui::ProgressBar::new(used_memory as f32 / total_memory as f32).show_percentage());
                }
                View::Network => {
                    ui.heading("Network Usage");
                    ui.separator();

                    for (interface_name, data) in self.networks.iter() {
                        ui.group(|ui| {
                            ui.label(format!("Interface: {}", interface_name));
                            ui.label(format!("Received: {} KB", data.received() / 1024));
                            ui.label(format!("Transmitted: {} KB", data.transmitted() / 1024));
                        });
                        ui.separator();
                    }

                    // TODO: Add ping results and WiFi SSID display
                }
                View::Terminal => {
                    ui.heading("Embedded Terminal");
                    ui.separator();

                    // Display terminal output
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        while let Some(output) = self.terminal.try_receive_output() {
                            ui.label(output);
                        }
                    });

                    // Input text box for commands
                    ui.horizontal(|ui| {
                        let input = ui.text_edit_singleline(&mut self.terminal_input);
                        if input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let cmd = self.terminal_input.trim().to_string();
                            if !cmd.is_empty() {
                                self.terminal.send_command(cmd);
                                self.terminal_input.clear();
                            }
                        }
                    });
                }
            }
        });

        // Set background color
        ctx.set_visuals(egui::Visuals {
            override_text_color: Some(egui::Color32::from_rgb(45, 45, 45)),
            window_fill: egui::Color32::from_rgb(245, 245, 245),
            ..egui::Visuals::light()
        });
    }
}
