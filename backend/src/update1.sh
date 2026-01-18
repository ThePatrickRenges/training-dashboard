#!/bin/bash

# Farben
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Admin Dashboard Update               ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

cd ~/training-system

# Backend erweitern
echo -e "${GREEN}📦 Aktualisiere Backend...${NC}"

cat > backend/Cargo.toml << 'EOF'
[package]
name = "training-backend"
version = "0.2.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
actix-cors = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.3"
tokio = { version = "1.35", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
bcrypt = "0.15"
uuid = { version = "1.6", features = ["v4", "serde"] }
EOF

echo -e "${YELLOW}   Backend Code wird heruntergeladen...${NC}"
echo -e "${YELLOW}   Bitte gehe zu: https://github.com/ThePatrickRenges/training-dashboard${NC}"
echo -e "${YELLOW}   Und lade die erweiterten Dateien herunter${NC}"

# Frontend erweitern
echo -e "${GREEN}📱 Aktualisiere Frontend...${NC}"

cat > frontend/Cargo.toml << 'EOF'
[package]
name = "training-dashboard"
version = "0.2.0"
edition = "2021"

[dependencies]
eframe = "0.27"
egui = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json", "blocking"] }
tokio = { version = "1.35", features = ["full"] }
EOF

# README Update
echo -e "${GREEN}📝 Aktualisiere README.md...${NC}"

cat >> README.md << 'EOF'

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

EOF

echo ""
echo -e "${GREEN}✅ Update-Struktur erstellt!${NC}"
echo ""
echo -e "${YELLOW}═══════════════════════════════════════${NC}"
echo -e "${BLUE}Nächste Schritte:${NC}"
echo -e "${YELLOW}═══════════════════════════════════════${NC}"
echo ""
echo "1. Ich erstelle dir jetzt die kompletten Code-Dateien"
echo "2. Du kopierst sie in die richtigen Ordner"
echo "3. Dann starten wir Backend & Frontend neu"
echo ""
echo -e "${GREEN}Bereit für die Code-Dateien? (j/n)${NC}"
EOF

chmod +x admin-update.sh

echo ""
echo -e "${GREEN}✅ Update-Script erstellt!${NC}"
echo ""
echo "Führe aus: ./admin-update.sh"
