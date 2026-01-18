#!/bin/bash

# Farben
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   GitHub Repository Setup             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# Zum Projektverzeichnis wechseln
cd ~/training-system

# .gitignore erstellen
echo -e "${GREEN}📝 Erstelle .gitignore${NC}"
cat > .gitignore << 'EOF'
# Rust
target/
Cargo.lock

# CSV Datenbank (optional - auskommentieren wenn du CSV hochladen willst)
backend/employees.csv

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Logs
*.log
EOF

# .gitattributes erstellen
echo -e "${GREEN}📝 Erstelle .gitattributes${NC}"
cat > .gitattributes << 'EOF'
# Auto detect text files and perform LF normalization
* text=auto

# Rust files
*.rs text eol=lf
Cargo.toml text eol=lf
Cargo.lock text eol=lf

# Shell scripts
*.sh text eol=lf
EOF

# LICENSE erstellen (MIT)
echo -e "${GREEN}📝 Erstelle LICENSE (MIT)${NC}"
cat > LICENSE << 'EOF'
MIT License

Copyright (c) 2026 Training Dashboard Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

# README.md erweitern
echo -e "${GREEN}📝 Erweitere README.md${NC}"
cat > README.md << 'EOF'
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
EOF

# Git initialisieren
echo -e "${GREEN}🔧 Initialisiere Git Repository${NC}"
git init

# Alle Dateien hinzufügen
echo -e "${GREEN}📦 Füge Dateien hinzu${NC}"
git add .

# Ersten Commit erstellen
echo -e "${GREEN}💾 Erstelle ersten Commit${NC}"
git commit -m "Initial commit: Training Dashboard System

- Backend mit Actix-Web und CSV-Datenbank
- Frontend mit egui/eframe Desktop GUI
- REST API für CRUD-Operationen
- Vollständige Dokumentation"

echo ""
echo -e "${YELLOW}════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Git Repository erfolgreich erstellt!${NC}"
echo -e "${YELLOW}════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}Nächste Schritte:${NC}"
echo ""
echo "1. Gehe zu GitHub: https://github.com/new"
echo "2. Erstelle ein neues Repository namens 'training-dashboard'"
echo "3. Führe folgende Befehle aus:"
echo ""
echo -e "${YELLOW}   git remote add origin https://github.com/DEIN-USERNAME/training-dashboard.git${NC}"
echo -e "${YELLOW}   git branch -M main${NC}"
echo -e "${YELLOW}   git push -u origin main${NC}"
echo ""
echo -e "${GREEN}Optional: Mit SSH statt HTTPS:${NC}"
echo -e "${YELLOW}   git remote add origin git@github.com:DEIN-USERNAME/training-dashboard.git${NC}"
echo ""
echo -e "${BLUE}Dateien im Repository:${NC}"
git ls-files
echo ""
echo -e "${GREEN}Viel Erfolg! 🚀${NC}"
