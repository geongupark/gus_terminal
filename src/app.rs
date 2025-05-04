use eframe::{egui, App, CreationContext, Storage};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    history: Vec<String>,
    #[serde(skip)]
    input: String,
    #[serde(skip)]
    command_history: Vec<String>,
    #[serde(skip)]
    history_index: Option<usize>,
    #[serde(skip)]
    force_focus: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        use chrono::Local;

        let now = Local::now();
        let time_string = now.format("Last login: %a %b %e %T on ttys000").to_string();

        Self {
            history: vec![time_string],
            input: String::new(),
            command_history: vec![],
            history_index: None,
            force_focus: true,
        }
    }
}


impl TemplateApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }

    fn execute_command(&mut self, cmd: &str) {
        if !cmd.trim().is_empty() {
            self.command_history.push(cmd.to_owned());
        }
        self.history_index = None;

        match cmd {
            "" => {}
            "ls" => {
                self.history.push("Documents\tLibrary\tMusic\tPublic\tDesktop".to_owned());
                self.history.push("Downloads\tMovies\tPictures".to_owned());
            }
            "pwd" => self.history.push("/Users/geongupark".to_owned()),
            "about" => {
                self.history.push("Geongu Park – System Software Engineer".to_owned());
                self.history.push("Experienced in ARM virtualization, embedded C++, and Rust WebAssembly.".to_owned());
            }
            "help" => self.history.push("Available commands: ls, pwd, about, help".to_owned()),
            other => self.history.push(format!("{}: command not found", other)),
        }
    }
}

impl App for TemplateApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if !cfg!(target_arch = "wasm32") {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.request_repaint();
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_resp = ui.interact(
                ui.max_rect(),
                ui.id().with("central_click"),
                egui::Sense::click(),
            );
            if panel_resp.clicked() {
                self.force_focus = true;
            }

            let light_green = egui::Color32::from_rgb(144, 238, 144);

            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for line in &self.history {
                        if ctx.style().visuals.dark_mode {
                            ui.colored_label(light_green, line);
                        } else {
                            ui.label(line);
                        }
                    }

                    ui.horizontal(|ui| {
                        let prompt = "geongupark@CV ~ % ";
                        if ctx.style().visuals.dark_mode {
                            ui.colored_label(light_green, prompt);
                        } else {
                            ui.label(prompt);
                        }

                        let input_id = ui.id().with("cmd_input");

                        if self.force_focus {
                            ui.memory_mut(|mem| mem.request_focus(input_id));
                            self.force_focus = false;
                        }

                        let mut text_edit = egui::TextEdit::singleline(&mut self.input)
                            .id(input_id)
                            .desired_width(ui.available_width())
                            .frame(false);

                        if ctx.style().visuals.dark_mode {
                            text_edit = text_edit.text_color(light_green);
                        }

                        let response = ui.add(text_edit);

                        let input = ctx.input(|i| i.clone());

                        // ↑
                        if input.key_pressed(egui::Key::ArrowUp) {
                            match self.history_index {
                                Some(0) | None if self.command_history.is_empty() => {}
                                Some(n) if n > 0 => self.history_index = Some(n - 1),
                                None => {
                                    self.history_index = Some(self.command_history.len().saturating_sub(1));
                                }
                                _ => {}
                            }

                            if let Some(idx) = self.history_index {
                                if let Some(cmd) = self.command_history.get(idx) {
                                    self.input = cmd.clone();
                                }
                            }
                        }

                        // ↓
                        if input.key_pressed(egui::Key::ArrowDown) {
                            match self.history_index {
                                Some(n) if n + 1 < self.command_history.len() => {
                                    self.history_index = Some(n + 1);
                                    self.input = self.command_history[n + 1].clone();
                                }
                                Some(_) => {
                                    self.history_index = None;
                                    self.input.clear();
                                }
                                None => {}
                            }
                        }

                        // Enter
                        if response.lost_focus() && input.key_pressed(egui::Key::Enter) {
                            let cmd = self.input.trim().to_owned();
                            self.history.push(format!("{}{}", prompt, cmd));
                            self.execute_command(&cmd);
                            self.input.clear();
                            self.force_focus = true;
                        }
                    });

                    ui.add_space(30.0);
                });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
                    ui.label(".");
                });
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
