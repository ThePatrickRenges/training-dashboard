use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
    training: String,
    duedate: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Gruen,
    Gelb,
    Rot,
}

impl Status {
    fn to_string(&self) -> &str {
        match self {
            Status::Gruen => "Aktuell (Grün)",
            Status::Gelb => "Bald fällig (Gelb)",
            Status::Rot => "Überfällig (Rot)",
        }
    }

    fn to_api_string(&self) -> &str {
        match self {
            Status::Gruen => "gruen",
            Status::Gelb => "gelb",
            Status::Rot => "rot",
        }
    }

    fn color(&self) -> egui::Color32 {
        match self {
            Status::Gruen => egui::Color32::from_rgb(34, 197, 94),
            Status::Gelb => egui::Color32::from_rgb(234, 179, 8),
            Status::Rot => egui::Color32::from_rgb(239, 68, 68),
        }
    }
}

#[derive(Serialize)]
struct CreateEmployee {
    name: String,
    training: String,
    duedate: String,
    status: String,
}

struct TrainingDashboard {
    employees: Vec<Employee>,
    new_name: String,
    new_training: String,
    new_duedate: String,
    new_status: Status,
    search_term: String,
    filter_critical: bool,
    message: Option<String>,
    api_url: String,
    rt: tokio::runtime::Runtime,
}

impl Default for TrainingDashboard {
    fn default() -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut app = Self {
            employees: Vec::new(),
            new_name: String::new(),
            new_training: String::new(),
            new_duedate: String::new(),
            new_status: Status::Gruen,
            search_term: String::new(),
            filter_critical: false,
            message: None,
            api_url: "http://127.0.0.1:8080".to_string(),
            rt,
        };
        app.load();
        app
    }
}

impl TrainingDashboard {
    fn load(&mut self) {
        match self.rt.block_on(async {
            reqwest::get(format!("{}/api/employees", self.api_url))
                .await?
                .json::<Vec<Employee>>()
                .await
        }) {
            Ok(employees) => {
                self.employees = employees;
                self.message = Some(format!("✓ {} Einträge geladen", self.employees.len()));
            }
            Err(e) => {
                self.message = Some(format!("✗ Verbindungsfehler: {}", e));
            }
        }
    }

    fn add_employee(&mut self) {
        if !self.new_name.is_empty() && !self.new_training.is_empty() && !self.new_duedate.is_empty() {
            let create_data = CreateEmployee {
                name: self.new_name.clone(),
                training: self.new_training.clone(),
                duedate: self.new_duedate.clone(),
                status: self.new_status.to_api_string().to_string(),
            };

            match self.rt.block_on(async {
                reqwest::Client::new()
                    .post(format!("{}/api/employees", self.api_url))
                    .json(&create_data)
                    .send()
                    .await
            }) {
                Ok(_) => {
                    self.new_name.clear();
                    self.new_training.clear();
                    self.new_duedate.clear();
                    self.new_status = Status::Gruen;
                    self.load();
                    self.message = Some("✓ Eintrag hinzugefügt".to_string());
                }
                Err(e) => {
                    self.message = Some(format!("✗ Fehler: {}", e));
                }
            }
        } else {
            self.message = Some("✗ Bitte alle Felder ausfüllen".to_string());
        }
    }

    fn delete_employee(&mut self, id: u32) {
        match self.rt.block_on(async {
            reqwest::Client::new()
                .delete(format!("{}/api/employees/{}", self.api_url, id))
                .send()
                .await
        }) {
            Ok(_) => {
                self.load();
                self.message = Some("✓ Eintrag gelöscht".to_string());
            }
            Err(e) => {
                self.message = Some(format!("✗ Fehler: {}", e));
            }
        }
    }

    fn filtered_employees(&self) -> Vec<Employee> {
        let mut filtered = self.employees.clone();

        if self.filter_critical {
            filtered.retain(|e| e.status != Status::Gruen);
        }

        if !self.search_term.is_empty() {
            let term = self.search_term.to_lowercase();
            filtered.retain(|e| {
                e.name.to_lowercase().contains(&term)
                    || e.training.to_lowercase().contains(&term)
                    || e.id.to_string().contains(&term)
            });
        }

        filtered
    }

    fn stats(&self) -> (usize, usize, usize, usize) {
        let total = self.employees.len();
        let green = self.employees.iter().filter(|e| e.status == Status::Gruen).count();
        let yellow = self.employees.iter().filter(|e| e.status == Status::Gelb).count();
        let red = self.employees.iter().filter(|e| e.status == Status::Rot).count();
        (total, green, yellow, red)
    }
}

impl eframe::App for TrainingDashboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎓 Training Dashboard");
            ui.label("Schulungen & Zertifikate verwalten (CSV Backend)");
            ui.add_space(10.0);

            let (total, green, yellow, red) = self.stats();
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", total)).size(32.0).strong());
                        ui.label("Gesamt");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", green)).size(32.0).color(Status::Gruen.color()).strong());
                        ui.label("Aktuell");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", yellow)).size(32.0).color(Status::Gelb.color()).strong());
                        ui.label("Bald fällig");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", red)).size(32.0).color(Status::Rot.color()).strong());
                        ui.label("Überfällig");
                    });
                });
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            if let Some(msg) = &self.message {
                ui.colored_label(
                    if msg.starts_with('✓') { egui::Color32::GREEN } else { egui::Color32::RED },
                    msg
                );
            }

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.heading("Neuen Eintrag anlegen");
                    ui.add_space(10.0);

                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.new_name);
                    ui.add_space(5.0);

                    ui.label("Training:");
                    ui.text_edit_singleline(&mut self.new_training);
                    ui.add_space(5.0);

                    ui.label("Fällig am (YYYY-MM-DD):");
                    ui.text_edit_singleline(&mut self.new_duedate);
                    ui.add_space(5.0);

                    ui.label("Status:");
                    egui::ComboBox::from_id_source("status_combo")
                        .selected_text(self.new_status.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_status, Status::Gruen, Status::Gruen.to_string());
                            ui.selectable_value(&mut self.new_status, Status::Gelb, Status::Gelb.to_string());
                            ui.selectable_value(&mut self.new_status, Status::Rot, Status::Rot.to_string());
                        });

                    ui.add_space(10.0);
                    if ui.button("➕ Hinzufügen").clicked() {
                        self.add_employee();
                    }
                });

                columns[1].group(|ui| {
                    ui.heading("Übersicht");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("🔍 Suche:");
                        ui.text_edit_singleline(&mut self.search_term);
                        if ui.button("🔄 Aktualisieren").clicked() {
                            self.load();
                        }
                    });
                    ui.checkbox(&mut self.filter_critical, "Nur kritische Einträge (Gelb/Rot)");
                    ui.add_space(10.0);

                    egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {
                        let filtered = self.filtered_employees();
                        
                        if filtered.is_empty() {
                            ui.label("Keine Einträge gefunden");
                        } else {
                            for emp in filtered {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("#{}", emp.id)).strong());
                                        ui.separator();
                                        
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(&emp.name).strong());
                                            ui.label(format!("Training: {}", emp.training));
                                            ui.label(format!("Fällig: {}", emp.duedate));
                                            ui.label(
                                                egui::RichText::new(emp.status.to_string())
                                                    .color(emp.status.color())
                                                    .strong()
                                            );
                                        });
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button("🗑").clicked() {
                                                self.delete_employee(emp.id);
                                            }
                                        });
                                    });
                                });
                                ui.add_space(5.0);
                            }
                        }
                    });
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Training Dashboard (CSV Backend)",
        options,
        Box::new(|_cc| Box::<TrainingDashboard>::default()),
    )
}
