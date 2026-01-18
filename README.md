# 🎓 Training Dashboard System

Ein vollständiges Schulungs- und Zertifikatsverwaltungssystem mit **Rust Backend** und **nativer Desktop GUI**.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge\&logo=rust\&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)

---

## ⬇️ Downloads

### Fedora / RPM (empfohlen)

Fertige RPM-Pakete findest du unter **Releases**:

👉 [https://github.com/ThePatrickRenges/training-dashboard/releases](https://github.com/ThePatrickRenges/training-dashboard/releases)

Installation (Beispiel v0.2.0):

```bash
sudo dnf install training-dashboard-frontend-0.2.0-1.fc43.x86_64.rpm
```

Start:

```bash
training-dashboard
```

---

## 📋 Übersicht

Dieses System ermöglicht die Verwaltung von Mitarbeiter-Schulungen und Zertifikaten mit Statusverfolgung:
**Aktuell / Bald fällig / Überfällig**.

### Features

* ✅ **CSV-Datenbank** für persistente Speicherung
* ✅ **REST API Backend** mit Actix-Web
* ✅ **Native Desktop GUI** mit egui/eframe
* ✅ **Echtzeit-Synchronisation** zwischen Frontend und Backend
* ✅ **Suche & Filter** Funktionalität
* ✅ **Status-Tracking** (Grün / Gelb / Rot)
* ✅ **CRUD-Operationen** (Create, Read, Update, Delete)

---

## 🏗️ Projektstruktur

```
training-system/
├── backend/              # REST API Server
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── employees.csv     # CSV Datenbank (automatisch erstellt)
├── frontend/             # Native Desktop GUI
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── screenshots/
├── start-backend.sh
├── start-frontend.sh
├── README.md
└── LICENSE
```

---

## 🚀 Lokale Entwicklung (ohne RPM)

### Voraussetzungen

* Rust (stable)
* Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Repository klonen

```bash
git clone https://github.com/ThePatrickRenges/training-dashboard.git
cd training-dashboard
```

### Backend starten

```bash
cd backend
cargo run --release
```

Backend läuft auf: `http://127.0.0.1:8080`

### Frontend starten (zweites Terminal)

```bash
cd frontend
cargo run --release
```

---

## 🔌 API Endpunkte

| Methode | Endpoint            | Beschreibung            |
| ------- | ------------------- | ----------------------- |
| GET     | /api/employees      | Alle Einträge abrufen   |
| POST    | /api/employees      | Neuen Eintrag erstellen |
| PUT     | /api/employees/{id} | Eintrag aktualisieren   |
| DELETE  | /api/employees/{id} | Eintrag löschen         |

### Beispiel (POST)

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

---

## 🛠️ Technologie-Stack

### Backend

* Actix-Web
* Tokio
* Serde
* CSV

### Frontend

* egui / eframe
* reqwest
* Tokio

---

## 📸 Screenshots

![Training Dashboard](https://raw.githubusercontent.com/ThePatrickRenges/training-dashboard/main/screenshots/dashboard.png)

**Enthält:**

* Statistik-Dashboard
* Formular zum Anlegen neuer Schulungen
* Tabellenansicht mit Suche & Filter
* Farbcodierte Statusanzeigen

---

## 🚧 Roadmap (geplant)

### Version 2.x

* Authentifizierung & Benutzerrollen
* Admin-Dashboard
* Rollenbasierte Zugriffskontrolle (RBAC)
* Vorbereitung für Active-Directory-Anbindung

---

## 📝 Lizenz

MIT-Lizenz – siehe [LICENSE](LICENSE)

---

## 🤝 Beitragen

1. Fork das Repository
2. Feature-Branch erstellen
3. Committen
4. Pull Request öffnen

---

## 📧 Kontakt

Bitte Issues für Bugs oder Feature-Wünsche verwenden.
