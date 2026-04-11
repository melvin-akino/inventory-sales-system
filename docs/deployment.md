# LumiSync — Deployment Guide

---

## Desktop Deployment (Recommended)

### Build

```bash
./setup.sh desktop
# or manually:
npm install
npm run tauri build
```

### Distribute

| Platform | Installer Location |
|----------|--------------------|
| Linux | `src-tauri/target/release/bundle/appimage/*.AppImage` |
| Linux (deb) | `src-tauri/target/release/bundle/deb/*.deb` |
| macOS | `src-tauri/target/release/bundle/dmg/*.dmg` |
| Windows | `src-tauri/target/release/bundle/msi/*.msi` |

Install on each machine by running the installer. The application self-contains all dependencies including SQLite.

### Per-Machine Data

Each installation has its own database. There is no shared/central database in desktop mode. For multi-machine shared data, use Web mode with a central server.

---

## Web Deployment

### Build Frontend

```bash
npm install
npm run build
# Output: dist/
```

### Serve with nginx

```nginx
server {
    listen 80;
    server_name yourdomain.com;
    root /var/www/lumisync/dist;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Optional: API proxy if using a backend
    location /api/ {
        proxy_pass http://localhost:3000/;
    }
}
```

### Run with serve (simple)

```bash
npm install -g serve
serve dist -l 8080
```

### Docker

```dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
```

```bash
docker build -t lumisync .
docker run -p 8080:80 lumisync
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | Web server port (web mode) |
| `TAURI_PLATFORM` | (auto) | Set by Tauri build system |

---

## Network Ports

| Mode | Port | Purpose |
|------|------|---------|
| Desktop dev | 1420 | Vite dev server |
| Web serve | 8080 | Static file server |

---

## Production Checklist

- [ ] Change default admin password
- [ ] Configure company name and TIN in Settings
- [ ] Set OR prefix to match your BIR-registered OR range
- [ ] Configure VAT rate (12% standard)
- [ ] Add all staff user accounts with appropriate roles
- [ ] Import initial product catalogue
- [ ] Test a complete sale and print a receipt
- [ ] Set up daily database backups
- [ ] Document the database file location for staff

---

## Updating

```bash
git pull origin main
npm install
./setup.sh desktop   # or ./setup.sh web
```

Database migrations run automatically on next startup.

---

## System Requirements

### Desktop

| Platform | Minimum | Recommended |
|----------|---------|-------------|
| CPU | Dual-core 1.6 GHz | Quad-core 2.0 GHz |
| RAM | 512 MB | 2 GB |
| Disk | 200 MB | 1 GB |
| OS | Win 10 / Ubuntu 18.04 / macOS 10.15 | Latest |

### Web Server

| Resource | Minimum |
|----------|---------|
| RAM | 512 MB |
| Disk | 1 GB |
| Node.js | 18 LTS |
