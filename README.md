# 🏠 Home Visual

A smart home 2D visualizer and controller. Design your home floorplan, place furniture and devices, simulate a robot vacuum cleaner, and control smart home devices — all from a single desktop/mobile app.

## ✨ Features

- **2D Floorplan Editor** — Draw rooms, place furniture and appliances with labels
- **Robot Cleaner Simulation** — Simulate vacuum cleaning paths room-by-room or full-flat
- **Smart Home Control** — Toggle lights, coffee machines, and other devices
- **Device Automation** — Create scenes and automations with triggers and conditions
- **Cross-Platform** — Desktop (macOS, Windows, Linux) and mobile (iOS, Android) via Tauri v2

## 🏗️ Architecture

```
home_redactor/
├── crates/
│   ├── shared/              # Domain models, enums, validation
│   ├── backend/             # Actix-web REST API + MongoDB + MQTT
│   └── bevy-visualizer/     # Bevy 2D engine (WASM) for floorplan & robot sim
├── tauri-app/               # Tauri v2 + React + TypeScript + Tailwind CSS
└── docs/                    # API documentation
```

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| App Shell | Tauri v2 |
| UI | React + TypeScript + Tailwind CSS + shadcn/ui |
| 2D Engine | Bevy (WASM) |
| Backend | Actix-web + Tokio |
| Database | MongoDB |
| Messaging | MQTT (rumqttc) |
| Smart Home | Home Assistant integration |

## 📋 Prerequisites

- **Rust** ≥ 1.80 (install via [rustup](https://rustup.rs/))
- **Node.js** ≥ 18 LTS
- **npm** ≥ 9
- **MongoDB** ≥ 7.0 (local or Atlas)
- **MQTT Broker** (e.g., Mosquitto)
- **WASM target**: `rustup target add wasm32-unknown-unknown`

## 🚀 Getting Started

### 1. Clone & Configure

```bash
git clone <repo-url> home_redactor
cd home_redactor
cp .env.example .env
# Edit .env with your MongoDB URI, MQTT broker, and JWT secret
```

### 2. Start the Backend

```bash
cargo run -p backend
# API available at http://localhost:8080
# Swagger UI at http://localhost:8080/swagger-ui/
```

### 3. Start the Tauri App (Development)

```bash
cd tauri-app
npm install
npm run tauri dev
```

### 4. Build for Production

```bash
# Backend
cargo build -p backend --release

# Tauri app
cd tauri-app
npm run tauri build
```

## 📡 API Endpoints

| Method | Path | Description |
|---|---|---|
| POST | `/api/auth/register` | Register a new user |
| POST | `/api/auth/login` | Login and get JWT token |
| GET/POST | `/api/homes` | List / create homes |
| GET/PUT/DELETE | `/api/homes/{id}` | Get / update / delete a home |
| GET/POST | `/api/rooms` | List / create rooms |
| GET/PUT/DELETE | `/api/rooms/{id}` | Get / update / delete a room |
| GET/POST | `/api/devices` | List / create devices |
| POST | `/api/devices/{id}/command` | Send command to device via MQTT |
| POST | `/api/robot/tasks` | Start robot cleaning task |
| GET | `/api/robot/tasks/{id}` | Get task status |
| GET/POST | `/api/scenes` | List / create scenes |
| POST | `/api/scenes/{id}/activate` | Activate a scene |
| GET/POST | `/api/automations` | List / create automations |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit changes: `git commit -am 'Add my feature'`
4. Push: `git push origin feature/my-feature`
5. Open a Pull Request

## 📄 License

MIT
