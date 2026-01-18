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
