use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
    training: String,
    duedate: String,
    status: Status,
    created_by: String,
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
            Status::Gruen => "Aktuell",
            Status::Gelb => "Bald fällig",
            Status::Rot => "Überfällig",
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    Admin,
    Manager,
    User,
}

impl Role {
    fn to_string(&self) -> &str {
        match self {
            Role::Admin => "Admin",
            Role::Manager => "Manager",
            Role::User => "User",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    role: Role,
    active: bool,
}

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Serialize)]
struct CreateEmployee {
    name: String,
    training: String,
    duedate: String,
    status: String,
}

#[derive(Serialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    role: Role,
}

#[derive(Serialize)]
struct UpdateUserRequest {
    active: Option<bool>,
    role: Option<Role>,
}

#[derive(PartialEq)]
enum Screen {
    Login,
    Dashboard,
    AdminPanel,
}

struct TrainingDashboard {
    screen: Screen,
    token: Option<String>,
    current_user: Option<User>,
    login_username: String,
    login_password: String,
    
    employees: Vec<Employee>,
    new_name: String,
    new_training: String,
    new_duedate: String,
    new_status: Status,
    search_term: String,
    filter_critical: bool,
    
    users: Vec<User>,
    new_user_username: String,
    new_user_password: String,
    new_user_role: Role,
    
    message: Option<String>,
    show_menu: bool,
    api_url: String,
    rt: tokio::runtime::Runtime,
}

impl Default for TrainingDashboard {
    fn default() -> Self {
        Self {
            screen: Screen::Login,
            token: None,
            current_user: None,
            login_username: String::new(),
            login_password: String::new(),
            
            employees: Vec::new(),
            new_name: String::new(),
            new_training: String::new(),
            new_duedate: String::new(),
            new_status: Status::Gruen,
            search_term: String::new(),
            filter_critical: false,
            
            users: Vec::new(),
            new_user_username: String::new(),
            new_user_password: String::new(),
            new_user_role: Role::User,
            
            message: None,
            show_menu: false,
            api_url: "http://127.0.0.1:8080".to_string(),
            rt: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl TrainingDashboard {
    fn login(&mut self) {
        let login_data = LoginRequest {
            username: self.login_username.clone(),
            password: self.login_password.clone(),
        };

        match self.rt.block_on(async {
            reqwest::Client::new()
                .post(format!("{}/api/auth/login", self.api_url))
                .json(&login_data)
                .send()
                .await?
                .json::<LoginResponse>()
                .await
        }) {
            Ok(response) => {
                self.token = Some(response.token);
                self.current_user = Some(response.user);
                self.screen = Screen::Dashboard;
                self.login_password.clear();
                self.load_employees();
                self.message = Some("✓ Angemeldet".to_string());
            }
            Err(_) => {
                self.message = Some("✗ Login fehlgeschlagen".to_string());
            }
        }
    }

    fn logout(&mut self) {
        self.token = None;
        self.current_user = None;
        self.screen = Screen::Login;
        self.employees.clear();
        self.users.clear();
        self.message = Some("✓ Abgemeldet".to_string());
    }

    fn load_employees(&mut self) {
        if let Some(token) = &self.token {
            match self.rt.block_on(async {
                reqwest::Client::new()
                    .get(format!("{}/api/employees", self.api_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?
                    .json::<Vec<Employee>>()
                    .await
            }) {
                Ok(employees) => self.employees = employees,
                Err(_) => self.message = Some("✗ Fehler beim Laden".to_string()),
            }
        }
    }

    fn add_employee(&mut self) {
        if !self.new_name.is_empty() && !self.new_training.is_empty() && !self.new_duedate.is_empty() {
            if let Some(token) = &self.token {
                let create_data = CreateEmployee {
                    name: self.new_name.clone(),
                    training: self.new_training.clone(),
                    duedate: self.new_duedate.clone(),
                    status: self.new_status.to_api_string().to_string(),
                };

                match self.rt.block_on(async {
                    reqwest::Client::new()
                        .post(format!("{}/api/employees", self.api_url))
                        .header("Authorization", format!("Bearer {}", token))
                        .json(&create_data)
                        .send()
                        .await
                }) {
                    Ok(_) => {
                        self.new_name.clear();
                        self.new_training.clear();
                        self.new_duedate.clear();
                        self.new_status = Status::Gruen;
                        self.load_employees();
                        self.message = Some("✓ Hinzugefügt".to_string());
                    }
                    Err(_) => self.message = Some("✗ Fehler".to_string()),
                }
            }
        }
    }

    fn delete_employee(&mut self, id: u32) {
        if let Some(token) = &self.token {
            match self.rt.block_on(async {
                reqwest::Client::new()
                    .delete(format!("{}/api/employees/{}", self.api_url, id))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
            }) {
                Ok(_) => {
                    self.load_employees();
                    self.message = Some("✓ Gelöscht".to_string());
                }
                Err(_) => self.message = Some("✗ Fehler".to_string()),
            }
        }
    }

    fn load_users(&mut self) {
        if let Some(token) = &self.token {
            match self.rt.block_on(async {
                reqwest::Client::new()
                    .get(format!("{}/api/users", self.api_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?
                    .json::<Vec<User>>()
                    .await
            }) {
                Ok(users) => self.users = users,
                Err(_) => self.message = Some("✗ Fehler".to_string()),
            }
        }
    }

    fn create_user(&mut self) {
        if !self.new_user_username.is_empty() && !self.new_user_password.is_empty() {
            if let Some(token) = &self.token {
                let create_data = CreateUserRequest {
                    username: self.new_user_username.clone(),
                    password: self.new_user_password.clone(),
                    role: self.new_user_role.clone(),
                };

                match self.rt.block_on(async {
                    reqwest::Client::new()
                        .post(format!("{}/api/users", self.api_url))
                        .header("Authorization", format!("Bearer {}", token))
                        .json(&create_data)
                        .send()
                        .await
                }) {
                    Ok(_) => {
                        self.new_user_username.clear();
                        self.new_user_password.clear();
                        self.load_users();
                        self.message = Some("✓ Benutzer erstellt".to_string());
                    }
                    Err(_) => self.message = Some("✗ Fehler".to_string()),
                }
            }
        }
    }

    fn toggle_user_active(&mut self, user_id: String, active: bool) {
        if let Some(token) = &self.token {
            let update_data = UpdateUserRequest {
                active: Some(!active),
                role: None,
            };

            match self.rt.block_on(async {
                reqwest::Client::new()
                    .put(format!("{}/api/users/{}", self.api_url, user_id))
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&update_data)
                    .send()
                    .await
            }) {
                Ok(_) => {
                    self.load_users();
                    self.message = Some("✓ Aktualisiert".to_string());
                }
                Err(_) => self.message = Some("✗ Fehler".to_string()),
            }
        }
    }

    fn delete_user(&mut self, user_id: String) {
        if let Some(token) = &self.token {
            match self.rt.block_on(async {
                reqwest::Client::new()
                    .delete(format!("{}/api/users/{}", self.api_url, user_id))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
            }) {
                Ok(_) => {
                    self.load_users();
                    self.message = Some("✓ Gelöscht".to_string());
                }
                Err(_) => self.message = Some("✗ Fehler".to_string()),
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

    fn is_admin(&self) -> bool {
        self.current_user.as_ref().map(|u| u.role == Role::Admin).unwrap_or(false)
    }

    fn is_manager_or_admin(&self) -> bool {
        self.current_user.as_ref().map(|u| {
            u.role == Role::Admin || u.role == Role::Manager
        }).unwrap_or(false)
    }
}

impl eframe::App for TrainingDashboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Login => self.render_login(ctx),
            Screen::Dashboard => self.render_dashboard(ctx),
            Screen::AdminPanel => self.render_admin_panel(ctx),
        }
    }
}

impl TrainingDashboard {
    fn render_login(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                ui.heading(egui::RichText::new("🎓 Training Dashboard").size(32.0));
                ui.label("Bitte anmelden");
                
                ui.add_space(30.0);
                
                egui::Frame::none()
                    .fill(egui::Color32::from_gray(30))
                    .rounding(10.0)
                    .inner_margin(30.0)
                    .show(ui, |ui| {
                        ui.set_max_width(400.0);
                        
                        ui.label("Benutzername:");
                        ui.text_edit_singleline(&mut self.login_username);
                        ui.add_space(10.0);
                        
                        ui.label("Passwort:");
                        let pwd = ui.add(egui::TextEdit::singleline(&mut self.login_password).password(true));
                        ui.add_space(20.0);
                        
                        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("Anmelden")).clicked() 
                            || (pwd.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            self.login();
                        }
                    });
                
                ui.add_space(20.0);
                
                if let Some(msg) = &self.message {
                    ui.colored_label(
                        if msg.starts_with('✓') { egui::Color32::GREEN } else { egui::Color32::RED },
                        msg
                    );
                }
                
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Standard: admin / admin123").small().weak());
            });
        });
    }

    fn render_dashboard(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("☰").clicked() {
                    self.show_menu = !self.show_menu;
                }
                
                ui.heading("🎓 Training Dashboard");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🚪").clicked() {
                        self.logout();
                    }
                    
                    if let Some(user) = &self.current_user {
                        ui.label(format!("👤 {} ({})", user.username, user.role.to_string()));
                    }
                });
            });
        });

        if self.show_menu {
            egui::SidePanel::left("menu").min_width(200.0).show(ctx, |ui| {
                ui.heading("Menü");
                ui.separator();
                
                if ui.button("📊 Dashboard").clicked() {
                    self.screen = Screen::Dashboard;
                    self.show_menu = false;
                }
                
                if self.is_admin() && ui.button("👥 Benutzerverwaltung").clicked() {
                    self.screen = Screen::AdminPanel;
                    self.load_users();
                    self.show_menu = false;
                }
                
                ui.separator();
                
                if ui.button("🚪 Abmelden").clicked() {
                    self.logout();
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(msg) = &self.message {
                ui.colored_label(
                    if msg.starts_with('✓') { egui::Color32::GREEN } else { egui::Color32::RED },
                    msg
                );
            }

            let (total, green, yellow, red) = self.stats();
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", total)).size(28.0).strong());
                        ui.label("Gesamt");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", green)).size(28.0).color(Status::Gruen.color()).strong());
                        ui.label("Aktuell");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", yellow)).size(28.0).color(Status::Gelb.color()).strong());
                        ui.label("Bald fällig");
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!("{}", red)).size(28.0).color(Status::Rot.color()).strong());
                        ui.label("Überfällig");
                    });
                });
            });

            ui.add_space(10.0);

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.heading("Neuer Eintrag");
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.new_name);
                    ui.label("Training:");
                    ui.text_edit_singleline(&mut self.new_training);
                    ui.label("Fällig (YYYY-MM-DD):");
                    ui.text_edit_singleline(&mut self.new_duedate);
                    ui.label("Status:");
                    egui::ComboBox::from_id_source("status")
                        .selected_text(self.new_status.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_status, Status::Gruen, Status::Gruen.to_string());
                            ui.selectable_value(&mut self.new_status, Status::Gelb, Status::Gelb.to_string());
                            ui.selectable_value(&mut self.new_status, Status::Rot, Status::Rot.to_string());
                        });
                    
                    if ui.button("➕ Hinzufügen").clicked() {
                        self.add_employee();
                    }
                });

                columns[1].group(|ui| {
                    ui.heading("Übersicht");
                    ui.horizontal(|ui| {
                        ui.label("🔍");
                        ui.text_edit_singleline(&mut self.search_term);
                        if ui.button("🔄").clicked() {
                            self.load_employees();
                        }
                    });
                    ui.checkbox(&mut self.filter_critical, "Nur Kritische");

                    egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {
                        for emp in self.filtered_employees() {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("#{}", emp.id));
                                    ui.separator();
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(&emp.name).strong());
                                        ui.label(format!("{} | {}", emp.training, emp.duedate));
                                        ui.label(egui::RichText::new(emp.status.to_string()).color(emp.status.color()));
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if self.is_manager_or_admin() && ui.button("🗑").clicked() {
                                            self.delete_employee(emp.id);
                                        }
                                    });
                                });
                            });
                        }
                    });
                });
            });
        });
    }

    fn render_admin_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("☰").clicked() {
                    self.show_menu = !self.show_menu;
                }
                
                ui.heading("👥 Admin-Panel");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🚪").clicked() {
                        self.logout();
                    }
                    
                    if let Some(user) = &self.current_user {
                        ui.label(format!("👤 {}", user.username));
                    }
                });
            });
        });

        if self.show_menu {
            egui::SidePanel::left("menu").min_width(200.0).show(ctx, |ui| {
                ui.heading("Menü");
                ui.separator();
                
                if ui.button("📊 Dashboard").clicked() {
                    self.screen = Screen::Dashboard;
                    self.show_menu = false;
                }
                
                if ui.button("👥 Benutzerverwaltung").clicked() {
                    self.screen = Screen::AdminPanel;
                    self.show_menu = false;
                }
                
                ui.separator();
                
                if ui.button("🚪 Abmelden").clicked() {
                    self.logout();
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(msg) = &self.message {
                ui.colored_label(
                    if msg.starts_with('✓') { egui::Color32::GREEN } else { egui::Color32::RED },
                    msg
                );
            }

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.heading("Neuer Benutzer");
                    ui.label("Username:");
                    ui.text_edit_singleline(&mut self.new_user_username);
                    ui.label("Passwort:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_user_password).password(true));
                    ui.label("Rolle:");
                    egui::ComboBox::from_id_source("role")
                        .selected_text(self.new_user_role.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_user_role, Role::Admin, "Admin");
                            ui.selectable_value(&mut self.new_user_role, Role::Manager, "Manager");
                            ui.selectable_value(&mut self.new_user_role, Role::User, "User");
                        });
                    
                    if ui.button("➕ Benutzer erstellen").clicked() {
                        self.create_user();
                    }

                    ui.add_space(20.0);
                    ui.separator();
                    ui.heading("Berechtigungen");
                    ui.label("👑 Admin: Alles");
                    ui.label("👔 Manager: Schulungen löschen");
                    ui.label("👤 User: Schulungen erstellen");
                });

                columns[1].group(|ui| {
                    ui.heading("Benutzer");
                    if ui.button("🔄 Aktualisieren").clicked() {
                        self.load_users();
                    }

                    egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
                        for user in self.users.clone() {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    let icon = match user.role {
                                        Role::Admin => "👑",
                                        Role::Manager => "👔",
                                        Role::User => "👤",
                                    };
                                    ui.label(icon);
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(&user.username).strong());
                                        ui.label(user.role.to_string());
                                        ui.label(if user.active { "✓ Aktiv" } else { "✗ Deaktiviert" });
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("🗑").clicked() {
                                            self.delete_user(user.id.clone());
                                        }
                                        if ui.button(if user.active { "⏸" } else { "▶" }).clicked() {
                                            self.toggle_user_active(user.id.clone(), user.active);
                                        }
                                    });
                                });
                            });
                        }
                    });
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Training Dashboard v2.0",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1200.0, 800.0]),
            ..Default::default()
        },
        Box::new(|_| Box::<TrainingDashboard>::default()),
    )
}