use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
    training: String,
    duedate: String,
    status: String,
}

#[derive(Deserialize)]
struct CreateEmployee {
    name: String,
    training: String,
    duedate: String,
    status: String,
}

struct AppState {
    employees: Mutex<Vec<Employee>>,
    next_id: Mutex<u32>,
    csv_path: String,
}

impl AppState {
    fn new(csv_path: &str) -> Self {
        let state = AppState {
            employees: Mutex::new(Vec::new()),
            next_id: Mutex::new(1),
            csv_path: csv_path.to_string(),
        };
        state.load_from_csv();
        state
    }

    fn load_from_csv(&self) {
        if let Ok(file) = File::open(&self.csv_path) {
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
                *self.next_id.lock().unwrap() = max_id + 1;
            }

            println!("✓ {} Einträge aus CSV geladen", employees.len());
        } else {
            println!("⚠ CSV-Datei nicht gefunden, erstelle neue");
            self.save_to_csv();
        }
    }

    fn save_to_csv(&self) {
        let employees = self.employees.lock().unwrap();
        
        if let Ok(file) = File::create(&self.csv_path) {
            let mut writer = csv::Writer::from_writer(file);
            
            for employee in employees.iter() {
                let _ = writer.serialize(employee);
            }
            
            let _ = writer.flush();
            println!("✓ Daten in CSV gespeichert");
        } else {
            eprintln!("✗ Fehler beim Speichern der CSV");
        }
    }
}

async fn get_employees(data: web::Data<AppState>) -> Result<HttpResponse> {
    let employees = data.employees.lock().unwrap();
    Ok(HttpResponse::Ok().json(&*employees))
}

async fn create_employee(
    employee: web::Json<CreateEmployee>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut employees = data.employees.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();
    
    let new_employee = Employee {
        id: *next_id,
        name: employee.name.clone(),
        training: employee.training.clone(),
        duedate: employee.duedate.clone(),
        status: employee.status.clone(),
    };
    
    *next_id += 1;
    employees.push(new_employee.clone());
    drop(employees);
    drop(next_id);
    
    data.save_to_csv();
    
    Ok(HttpResponse::Ok().json(new_employee))
}

async fn update_employee(
    path: web::Path<u32>,
    employee: web::Json<Employee>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut employees = data.employees.lock().unwrap();
    let id = path.into_inner();
    
    if let Some(pos) = employees.iter().position(|e| e.id == id) {
        employees[pos] = employee.into_inner();
        let result = employees[pos].clone();
        drop(employees);
        
        data.save_to_csv();
        Ok(HttpResponse::Ok().json(result))
    } else {
        Ok(HttpResponse::NotFound().body("Employee not found"))
    }
}

async fn delete_employee(
    path: web::Path<u32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut employees = data.employees.lock().unwrap();
    let id = path.into_inner();
    
    if let Some(pos) = employees.iter().position(|e| e.id == id) {
        employees.remove(pos);
        drop(employees);
        
        data.save_to_csv();
        Ok(HttpResponse::Ok().body("Deleted"))
    } else {
        Ok(HttpResponse::NotFound().body("Employee not found"))
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Backend läuft!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║   Training Dashboard Backend          ║");
    println!("║   CSV-Datenbank Backend                ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    println!("🚀 Server startet auf http://127.0.0.1:8080");
    println!("📁 CSV-Datei: employees.csv");
    println!();
    
    let app_state = web::Data::new(AppState::new("employees.csv"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .route("/", web::get().to(health_check))
            .route("/api/employees", web::get().to(get_employees))
            .route("/api/employees", web::post().to(create_employee))
            .route("/api/employees/{id}", web::put().to(update_employee))
            .route("/api/employees/{id}", web::delete().to(delete_employee))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
