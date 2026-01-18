use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware, HttpRequest};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs::File;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

// Models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Role {
    Admin,
    Manager,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    #[serde(skip_serializing)]
    password_hash: String,
    role: Role,
    active: bool,
    created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
    training: String,
    duedate: String,
    status: String,
    created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Session {
    token: String,
    user_id: String,
    username: String,
    role: Role,
    created_at: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: UserResponse,
}

#[derive(Serialize)]
struct UserResponse {
    id: String,
    username: String,
    role: Role,
    active: bool,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    role: Role,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    active: Option<bool>,
    role: Option<Role>,
}

#[derive(Deserialize)]
struct CreateEmployee {
    name: String,
    training: String,
    duedate: String,
    status: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
    employees: Mutex<Vec<Employee>>,
    sessions: Mutex<Vec<Session>>,
    next_employee_id: Mutex<u32>,
}

impl AppState {
    fn new() -> Self {
        let state = AppState {
            users: Mutex::new(Vec::new()),
            employees: Mutex::new(Vec::new()),
            sessions: Mutex::new(Vec::new()),
            next_employee_id: Mutex::new(1),
        };
        
        state.create_default_admin();
        state.load_employees();
        state
    }

    fn create_default_admin(&self) {
        let mut users = self.users.lock().unwrap();
        let admin = User {
            id: Uuid::new_v4().to_string(),
            username: "admin".to_string(),
            password_hash: hash("admin123", DEFAULT_COST).unwrap(),
            role: Role::Admin,
            active: true,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        users.push(admin);
        println!("✓ Admin erstellt (Username: admin, Passwort: admin123)");
    }

    fn load_employees(&self) {
        if let Ok(file) = File::open("employees.csv") {
            let mut reader = csv::Reader::from_reader(file);
            let mut employees = self.employees.lock().unwrap();
            let mut max_id = 0;

            for result in reader.deserialize() {
                if let Ok(employee) = result {
                    let emp: Employee = employee;
                    if emp.id > max_id {
                        max_id = emp.id;
                    }
                    employees.push(emp);
                }
            }

            if max_id > 0 {
                *self.next_employee_id.lock().unwrap() = max_id + 1;
            }
            println!("✓ {} Schulungen geladen", employees.len());
        }
    }

    fn save_employees(&self) {
        let employees = self.employees.lock().unwrap();
        if let Ok(file) = File::create("employees.csv") {
            let mut writer = csv::Writer::from_writer(file);
            for employee in employees.iter() {
                let _ = writer.serialize(employee);
            }
            let _ = writer.flush();
        }
    }

    fn verify_token(&self, token: &str) -> Option<Session> {
        let sessions = self.sessions.lock().unwrap();
        sessions.iter().find(|s| s.token == token).cloned()
    }

    fn check_permission(&self, session: &Session, required_role: Role) -> bool {
        match (&session.role, &required_role) {
            (Role::Admin, _) => true,
            (Role::Manager, Role::User) => true,
            (Role::Manager, Role::Manager) => true,
            (Role::User, Role::User) => true,
            _ => false,
        }
    }
}

async fn login(login_req: web::Json<LoginRequest>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let users = data.users.lock().unwrap();
    
    if let Some(user) = users.iter().find(|u| u.username == login_req.username) {
        if !user.active {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({"error": "Benutzer deaktiviert"})));
        }

        if verify(&login_req.password, &user.password_hash).unwrap_or(false) {
            let token = Uuid::new_v4().to_string();
            let user_id = user.id.clone();
            let username = user.username.clone();
            let role = user.role.clone();
            let active = user.active;
            
            drop(users);
            
            let session = Session {
                token: token.clone(),
                user_id: user_id.clone(),
                username: username.clone(),
                role: role.clone(),
                created_at: chrono::Utc::now().to_rfc3339(),
            };

            data.sessions.lock().unwrap().push(session);

            return Ok(HttpResponse::Ok().json(LoginResponse {
                token: token.clone(),
                user: UserResponse {
                    id: user_id,
                    username,
                    role,
                    active,
                },
            }));
        }
    }

    Ok(HttpResponse::Unauthorized().json(serde_json::json!({"error": "Ungültige Anmeldedaten"})))
}

async fn logout(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        let mut sessions = data.sessions.lock().unwrap();
        sessions.retain(|s| s.token != session.token);
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Abgemeldet"})))
}

async fn get_current_user(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        let users = data.users.lock().unwrap();
        if let Some(user) = users.iter().find(|u| u.id == session.user_id) {
            return Ok(HttpResponse::Ok().json(UserResponse {
                id: user.id.clone(),
                username: user.username.clone(),
                role: user.role.clone(),
                active: user.active,
            }));
        }
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn get_users(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        if !data.check_permission(&session, Role::Manager) {
            return Ok(HttpResponse::Forbidden().finish());
        }

        let users = data.users.lock().unwrap();
        let user_list: Vec<UserResponse> = users.iter().map(|u| UserResponse {
            id: u.id.clone(),
            username: u.username.clone(),
            role: u.role.clone(),
            active: u.active,
        }).collect();

        return Ok(HttpResponse::Ok().json(user_list));
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn create_user(req: HttpRequest, user_req: web::Json<CreateUserRequest>, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        if !data.check_permission(&session, Role::Admin) {
            return Ok(HttpResponse::Forbidden().finish());
        }

        let mut users = data.users.lock().unwrap();
        
        if users.iter().any(|u| u.username == user_req.username) {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Username existiert"})));
        }

        let new_user = User {
            id: Uuid::new_v4().to_string(),
            username: user_req.username.clone(),
            password_hash: hash(&user_req.password, DEFAULT_COST).unwrap(),
            role: user_req.role.clone(),
            active: true,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        users.push(new_user.clone());

        return Ok(HttpResponse::Ok().json(UserResponse {
            id: new_user.id,
            username: new_user.username,
            role: new_user.role,
            active: new_user.active,
        }));
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn update_user(req: HttpRequest, path: web::Path<String>, update_req: web::Json<UpdateUserRequest>, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        if !data.check_permission(&session, Role::Admin) {
            return Ok(HttpResponse::Forbidden().finish());
        }

        let mut users = data.users.lock().unwrap();
        let user_id = path.into_inner();

        if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
            if let Some(active) = update_req.active {
                user.active = active;
            }
            if let Some(role) = &update_req.role {
                user.role = role.clone();
            }

            let response = UserResponse {
                id: user.id.clone(),
                username: user.username.clone(),
                role: user.role.clone(),
                active: user.active,
            };

            return Ok(HttpResponse::Ok().json(response));
        }
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn delete_user(req: HttpRequest, path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        if !data.check_permission(&session, Role::Admin) {
            return Ok(HttpResponse::Forbidden().finish());
        }

        let mut users = data.users.lock().unwrap();
        let user_id = path.into_inner();

        if user_id == session.user_id {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Kann eigenen Account nicht löschen"})));
        }

        users.retain(|u| u.id != user_id);
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn get_employees(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse> {
    if extract_session(&req, &data).is_some() {
        let employees = data.employees.lock().unwrap();
        return Ok(HttpResponse::Ok().json(&*employees));
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn create_employee(req: HttpRequest, employee: web::Json<CreateEmployee>, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        let mut employees = data.employees.lock().unwrap();
        let mut next_id = data.next_employee_id.lock().unwrap();

        let new_employee = Employee {
            id: *next_id,
            name: employee.name.clone(),
            training: employee.training.clone(),
            duedate: employee.duedate.clone(),
            status: employee.status.clone(),
            created_by: session.username.clone(),
        };

        *next_id += 1;
        employees.push(new_employee.clone());
        drop(employees);
        drop(next_id);

        data.save_employees();
        return Ok(HttpResponse::Ok().json(new_employee));
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn update_employee(req: HttpRequest, path: web::Path<u32>, employee: web::Json<Employee>, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        if !data.check_permission(&session, Role::User) {
            return Ok(HttpResponse::Forbidden().finish());
        }

        let mut employees = data.employees.lock().unwrap();
        let id = path.into_inner();

        if let Some(pos) = employees.iter().position(|e| e.id == id) {
            employees[pos] = employee.into_inner();
            let result = employees[pos].clone();
            drop(employees);

            data.save_employees();
            return Ok(HttpResponse::Ok().json(result));
        }
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn delete_employee(req: HttpRequest, path: web::Path<u32>, data: web::Data<AppState>) -> Result<HttpResponse> {
    if let Some(session) = extract_session(&req, &data) {
        if !data.check_permission(&session, Role::Manager) {
            return Ok(HttpResponse::Forbidden().finish());
        }

        let id = path.into_inner();
        let mut employees = data.employees.lock().unwrap();
        employees.retain(|e| e.id != id);
        drop(employees);

        data.save_employees();
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::Unauthorized().finish())
}

fn extract_session(req: &HttpRequest, data: &web::Data<AppState>) -> Option<Session> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .and_then(|token| data.verify_token(token))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║   Training Dashboard Backend v2.0     ║");
    println!("╚════════════════════════════════════════╝");
    println!("🚀 Server: http://127.0.0.1:8080");
    println!();

    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .route("/api/auth/login", web::post().to(login))
            .route("/api/auth/logout", web::post().to(logout))
            .route("/api/auth/me", web::get().to(get_current_user))
            .route("/api/users", web::get().to(get_users))
            .route("/api/users", web::post().to(create_user))
            .route("/api/users/{id}", web::put().to(update_user))
            .route("/api/users/{id}", web::delete().to(delete_user))
            .route("/api/employees", web::get().to(get_employees))
            .route("/api/employees", web::post().to(create_employee))
            .route("/api/employees/{id}", web::put().to(update_employee))
            .route("/api/employees/{id}", web::delete().to(delete_employee))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}