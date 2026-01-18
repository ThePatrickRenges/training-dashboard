# 🎓 Training Dashboard System

Ein vollständiges Schulungs- und Zertifikatsverwaltungssystem mit Rust Backend und nativer Desktop GUI.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)

## 📋 Übersicht

Dieses System ermöglicht die Verwaltung von Mitarbeiter-Schulungen und Zertifikaten mit Statusverfolgung (Aktuell/Bald fällig/Überfällig).

### Features

- ✅ **CSV-Datenbank** für persistente Speicherung
- ✅ **REST API Backend** mit Actix-Web
- ✅ **Native Desktop GUI** mit egui/eframe
- ✅ **Echtzeit-Synchronisation** zwischen Frontend und Backend
- ✅ **Suche & Filter** Funktionalität
- ✅ **Status-Tracking** (Grün/Gelb/Rot)
- ✅ **CRUD-Operationen** (Create, Read, Update, Delete)

## 🏗️ Projektstruktur

```
training-system/
├── backend/              # REST API Server
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── employees.csv     # CSV Datenbank (automatisch erstellt)
├── frontend/             # Desktop GUI Anwendung
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── README.md
├── LICENSE
├── start-backend.sh      # Backend Start-Script
└── start-frontend.sh     # Frontend Start-Script
```

## 🚀 Installation & Start

### Voraussetzungen

- Rust (neueste stabile Version)
- Cargo

```bash
# Rust installieren (falls nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Repository klonen

```bash
git clone https://github.com/IHR-USERNAME/training-dashboard.git
cd training-dashboard
```

### Backend starten

```bash
cd backend
cargo run --release
```

Das Backend läuft auf: `http://127.0.0.1:8080`

### Frontend starten (neues Terminal)

```bash
cd frontend
cargo run --release
```

### Mit Start-Scripten (Linux/macOS)

```bash
# Terminal 1: Backend
./start-backend.sh

# Terminal 2: Frontend
./start-frontend.sh
```

## 🔌 API Endpunkte

| Methode | Endpoint | Beschreibung |
|---------|----------|--------------|
| `GET` | `/api/employees` | Alle Einträge abrufen |
| `POST` | `/api/employees` | Neuen Eintrag erstellen |
| `PUT` | `/api/employees/{id}` | Eintrag aktualisieren |
| `DELETE` | `/api/employees/{id}` | Eintrag löschen |

### Beispiel Request (POST)

```bash
curl -X POST http://127.0.0.1:8080/api/employees \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Max Mustermann",
    "training": "Erste Hilfe",
    "duedate": "2026-12-31",
    "status": "gruen"
  }'
```

## 🛠️ Technologie-Stack

### Backend
- **Framework:** Actix-Web 4.4
- **Serialisierung:** Serde
- **Datenbank:** CSV (csv crate)
- **Async Runtime:** Tokio

### Frontend
- **GUI Framework:** egui 0.27 / eframe
- **HTTP Client:** reqwest
- **Async Runtime:** Tokio

## 📊 Datenmodell

```rust
struct Employee {
    id: u32,
    name: String,
    training: String,
    duedate: String,      // Format: YYYY-MM-DD
    status: String,       // "gruen", "gelb", "rot"
}
```

## 🎨 Screenshots

### Desktop Anwendung
Die GUI zeigt:
- Statistik-Dashboard mit Gesamtzahlen
- Formular zum Hinzufügen neuer Einträge
- Übersichtstabelle mit Such- und Filterfunktion
- Farbcodierte Status-Anzeigen

## 📝 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](LICENSE) für Details.

## 🤝 Beitragen

Contributions sind willkommen! Bitte:

1. Fork das Repository
2. Erstelle einen Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit deine Änderungen (`git commit -m 'Add some AmazingFeature'`)
4. Push zum Branch (`git push origin feature/AmazingFeature`)
5. Öffne einen Pull Request

## 📧 Kontakt

Bei Fragen oder Problemen bitte ein Issue erstellen.

## 🙏 Danksagungen

- [Actix-Web](https://actix.rs/) - Web Framework
- [egui](https://www.egui.rs/) - GUI Framework
- Rust Community

---

**Hinweis:** Beim ersten Build werden alle Dependencies heruntergeladen, was einige Minuten dauern kann.

## 📸 Screenshots

### Desktop Anwendung

![Training Dashboard](https://raw.githubusercontent.com/ThePatrickRenges/training-dashboard/main/screenshots/dashboard.png)

**Features im Screenshot:**
- 📊 **Statistik-Dashboard** - Übersicht über alle Trainings (Gesamt, Aktuell, Bald fällig, Überfällig)
- ➕ **Neuen Eintrag anlegen** - Formular zum Hinzufügen von Mitarbeiter-Schulungen
- 📋 **Übersicht** - Tabellarische Darstellung aller Einträge
- 🔍 **Suche & Filter** - Filtern nach kritischen Einträgen (Gelb/Rot)
- 🔄 **Echtzeit-Synchronisation** - Automatische Aktualisierung vom Backend
- 🟢🟡🔴 **Farbcodierte Status** - Visuelles Status-Tracking

### Backend Terminal

Das Backend läuft als REST API Server und speichert alle Daten in einer CSV-Datei:
```
╔════════════════════════════════════════╗
║   Training Dashboard Backend          ║
║   CSV-Datenbank Backend                ║
╚════════════════════════════════════════╝

🚀 Server startet auf http://127.0.0.1:8080
📁 CSV-Datei: employees.csv
```

### Native Desktop GUI

Die Anwendung nutzt **egui/eframe** für eine native Desktop-Erfahrung:
- ✅ Läuft auf Windows, macOS und Linux
- ✅ Keine Webbrowser erforderlich
- ✅ Schnelle Performance
- ✅ Natives Look & Feel


## 📸 Screenshots

### Desktop Anwendung

![Training Dashboard](https://raw.githubusercontent.com/ThePatrickRenges/training-dashboard/main/screenshots/dashboard.png)

**Features im Screenshot:**
- 📊 **Statistik-Dashboard** - Übersicht über alle Trainings (Gesamt, Aktuell, Bald fällig, Überfällig)
- ➕ **Neuen Eintrag anlegen** - Formular zum Hinzufügen von Mitarbeiter-Schulungen
- 📋 **Übersicht** - Tabellarische Darstellung aller Einträge
- 🔍 **Suche & Filter** - Filtern nach kritischen Einträgen (Gelb/Rot)
- 🔄 **Echtzeit-Synchronisation** - Automatische Aktualisierung vom Backend
- 🟢🟡🔴 **Farbcodierte Status** - Visuelles Status-Tracking

### Backend Terminal

Das Backend läuft als REST API Server und speichert alle Daten in einer CSV-Datei:
```
╔════════════════════════════════════════╗
║   Training Dashboard Backend          ║
║   CSV-Datenbank Backend                ║
╚════════════════════════════════════════╝

🚀 Server startet auf http://127.0.0.1:8080
📁 CSV-Datei: employees.csv
```

### Native Desktop GUI

Die Anwendung nutzt **egui/eframe** für eine native Desktop-Erfahrung:
- ✅ Läuft auf Windows, macOS und Linux
- ✅ Keine Webbrowser erforderlich
- ✅ Schnelle Performance
- ✅ Natives Look & Feel


## 🆕 Version 2.0 - Admin Dashboard

### Neue Features

#### 🔐 Authentifizierung & Berechtigungen
- **Login-System** mit Token-basierter Authentifizierung
- **3 Benutzerrollen**: Administrator, Manager, Benutzer
- **Session-Management** mit sicheren Tokens

#### 👥 Benutzerverwaltung
- Benutzer erstellen, bearbeiten, löschen
- Rollen zuweisen (Admin, Manager, User)
- Benutzer aktivieren/deaktivieren
- Passwort-Hashing mit bcrypt

#### 📊 Admin-Dashboard
- **Hamburger-Menü** für Navigation
- Benutzer-Übersicht mit Rollenverwaltung
- Berechtigungssystem
- Aktivitäts-Tracking (wer hat was erstellt)

### Berechtigungen

| Berechtigung | Admin | Manager | User |
|--------------|-------|---------|------|
| Benutzer erstellen/löschen | ✅ | ❌ | ❌ |
| Benutzer bearbeiten | ✅ | ❌ | ❌ |
| Schulungen löschen | ✅ | ✅ | ❌ |
| Schulungen bearbeiten | ✅ | ✅ | ✅ |
| Schulungen erstellen | ✅ | ✅ | ✅ |
| Schulungen ansehen | ✅ | ✅ | ✅ |

### Standard-Login

Bei der ersten Installation wird automatisch ein Admin-Account erstellt:

- **Benutzername:** `admin`
- **Passwort:** `admin123`

⚠️ **Wichtig:** Bitte ändern Sie das Passwort nach dem ersten Login!

### Neue API-Endpunkte

#### Authentifizierung
- `POST /api/auth/login` - Login
- `POST /api/auth/logout` - Logout
- `GET /api/auth/me` - Aktueller Benutzer

#### Benutzerverwaltung (nur Admin)
- `GET /api/users` - Alle Benutzer auflisten
- `POST /api/users` - Benutzer erstellen
- `PUT /api/users/{id}` - Benutzer aktualisieren
- `DELETE /api/users/{id}` - Benutzer löschen

### Sicherheit

- ✅ Passwörter werden mit bcrypt gehasht
- ✅ Token-basierte Authentifizierung
- ✅ Rollenbasierte Zugriffskontrolle (RBAC)
- ✅ Session-Management
- ✅ CORS-Konfiguration

