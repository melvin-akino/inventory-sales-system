# LumiSync - Inventory & Sales System

A full-featured inventory and sales management system for the Philippines market, supporting both web and desktop (Tauri) deployments.

## Quick Start

### Prerequisites
- **Node.js 18+** (for frontend)
- **Rust 1.70+** (for desktop/Tauri builds only)
- **Docker & Docker Compose** (for backend - recommended)

### Default Login
- **Username:** `admin`
- **Password:** `Admin@123`

---

## Running the Application

### 1. **Web Version (Recommended for Quick Setup)**

Start the backend and web server:

```bash
# Start backend with Docker
docker-compose up

# In another terminal, install dependencies and build
npm install
npm run build

# Start web server on port 8080
npx serve dist -l 8080
```

Or use the setup script:

```bash
./setup.sh web
```

Then access at: **http://localhost:8080**

---

### 2. **Desktop Version (Tauri)**

Build and run the desktop application:

```bash
./setup.sh desktop
```

This will:
1. Check system dependencies (Node.js, Rust, Tauri)
2. Install Linux libraries (if on Linux)
3. Build the desktop app with embedded backend
4. Create an installer in `src-tauri/target/release/bundle/`

To run directly without building an installer:

```bash
npm install
cargo build --release -p lumisync
./src-tauri/target/release/lumisync
```

---

### 3. **Development Mode (Hot Reload)**

#### Desktop Development:
```bash
./setup.sh dev
```

This starts:
- Frontend on http://localhost:1420 (with hot reload)
- Desktop app with embedded database

#### Web Development:
```bash
./setup.sh dev-web
```

This starts:
- Frontend on http://localhost:1420 (with hot reload)
- Backend on http://localhost:3000 (via Docker)
- API proxy configured

---

## Architecture

### Web Mode
- **Frontend:** Vue 3 + Vite (SPA)
- **Backend:** Rust/Axum server (REST API)
- **Database:** SQLite (persistent volume in Docker)
- **Server:** Nginx (static file serving in production)

### Desktop Mode (Tauri)
- **Frontend:** Vue 3 + Vite
- **Backend:** Rust/Tauri commands (IPC - no HTTP)
- **Database:** SQLite (embedded in app)
- **Bundling:** Native installers for Windows, macOS, Linux

---

## Docker Services

The application includes three services:

```yaml
Services:
  backend   - Rust API server (port 3000)
  frontend  - Nginx web server (port 8080)
  Volume    - lumisync-data: persistent database storage
```

### Start Services
```bash
docker-compose up
```

### Stop Services
```bash
docker-compose down
```

### View Logs
```bash
docker-compose logs -f backend
docker-compose logs -f frontend
```

---

## Project Structure

```
.
├── src/                    # Vue 3 frontend source
│   ├── components/         # Reusable Vue components
│   ├── views/             # Page components
│   ├── stores/            # Pinia state management
│   ├── utils/             # API client, helpers
│   └── router/            # Vue Router configuration
│
├── src-tauri/             # Tauri desktop app
│   ├── src/               # Rust backend code
│   │   ├── commands/      # IPC command handlers
│   │   ├── db/            # Database logic
│   │   └── models/        # Data models
│   └── Cargo.toml         # Rust dependencies
│
├── server/                # Rust REST API (web mode)
│   ├── src/               # API implementation
│   └── Cargo.toml
│
├── docker-compose.yml     # Service orchestration
├── Dockerfile.backend     # Backend container
├── Dockerfile.frontend    # Frontend container
├── setup.sh              # One-command setup script
└── package.json          # Node dependencies
```

---

## Features

- **User Management** - Multi-role support (Admin, Manager, Cashier)
- **Product Inventory** - Category management, stock tracking
- **Sales & Invoicing** - POS interface, invoice generation
- **Customers & Suppliers** - Contact management
- **Reports** - Sales, inventory, profit/loss, VAT
- **Dashboard** - Real-time statistics and summaries

---

## Development

### Running Frontend Only
```bash
npm install
npm run dev
# Access at http://localhost:1420
```

### Building Frontend Assets
```bash
npm run build
# Output: dist/
```

### Backend API Health Check
```bash
curl http://localhost:3000/health
```

### Tauri Commands (Desktop)
All backend operations use Tauri IPC in desktop mode. Commands are in `src-tauri/src/commands/`.

---

## Troubleshooting

### Backend Won't Start
```bash
# Check if port 3000 is in use
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows

# Kill existing process and restart
docker-compose restart backend
```

### Frontend Can't Reach Backend
- **Web mode:** Ensure backend is running (`docker-compose up`)
- **Dev mode:** Check Vite proxy config in `vite.config.js`
- **Desktop:** Verify Tauri IPC commands are properly invoked

### Build Failures
- Clear npm cache: `npm cache clean --force`
- Reinstall: `rm -rf node_modules && npm install`
- Check Node version: `node --version` (should be 18+)

### On Linux (Tauri Build)
Missing libraries will be installed automatically. If issues persist:
```bash
sudo apt-get install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

---

## Deployment

### Production Web Deployment
```bash
npm run build
docker build -f Dockerfile.frontend -t lumisync:latest .
docker push lumisync:latest
```

Then deploy with Docker Compose or Kubernetes.

### Desktop Distribution
Find installers in:
```
src-tauri/target/release/bundle/
```

- **Windows:** `.msi` installer
- **macOS:** `.dmg` or `.app`
- **Linux:** `.deb`, `.rpm`, or AppImage

---

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review Docker logs: `docker-compose logs`
3. Check console errors in browser DevTools (web mode)

---

**Default Credentials:**
- Username: `admin`
- Password: `Admin@123`

⚠️ Change these in production!
