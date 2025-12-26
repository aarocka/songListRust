# songListRust

A rewrite of my node.js express app but now in rust. This repository contains both the original Node.js Express application and the new Rust implementation, both configured to run in Docker containers and GitHub Codespaces.

## Project Structure

```
.
├── nodejs-app/          # Original Node.js Express application
│   ├── index.js        # Express server
│   ├── package.json    # Node.js dependencies
│   └── Dockerfile      # Docker configuration for Node.js
├── rust-app/           # Rust port of the application
│   ├── src/
│   │   └── main.rs     # Rust web server (Actix-web)
│   ├── Cargo.toml      # Rust dependencies
│   └── Dockerfile      # Docker configuration for Rust
├── .devcontainer/      # GitHub Codespaces configuration
│   └── devcontainer.json
└── docker-compose.yml  # Orchestrates both applications
```

## Features

Both applications implement the same Song List API with the following endpoints:

- `GET /` - Welcome message
- `GET /songs` - List all songs
- `GET /songs/:id` - Get a specific song
- `POST /songs` - Create a new song
- `DELETE /songs/:id` - Delete a song

## Getting Started

### Using GitHub Codespaces (Recommended)

1. Open this repository in GitHub Codespaces
2. The environment will automatically set up with both Node.js and Rust tools
3. Dependencies will be installed automatically
4. Run the applications:
   - Node.js app: `cd nodejs-app && npm start` (runs on port 3000)
   - Rust app: `cd rust-app && cargo run` (runs on port 8080)

### Using Docker Compose

```bash
# Build and run both applications
docker-compose up --build

# Run in detached mode
docker-compose up -d

# Stop all services
docker-compose down
```

The applications will be available at:
- Node.js app: http://localhost:3000
- Rust app: http://localhost:8080

### Running Individually

#### Node.js Application

```bash
cd nodejs-app
npm install
npm start
```

#### Rust Application

```bash
cd rust-app
cargo build
cargo run
```

## API Usage Examples

### Get all songs
```bash
# Node.js app
curl http://localhost:3000/songs

# Rust app
curl http://localhost:8080/songs
```

### Create a new song
```bash
# Node.js app
curl -X POST http://localhost:3000/songs \
  -H "Content-Type: application/json" \
  -d '{"title":"Imagine","artist":"John Lennon","year":1971}'

# Rust app
curl -X POST http://localhost:8080/songs \
  -H "Content-Type: application/json" \
  -d '{"title":"Imagine","artist":"John Lennon","year":1971}'
```

### Delete a song
```bash
# Node.js app
curl -X DELETE http://localhost:3000/songs/1

# Rust app
curl -X DELETE http://localhost:8080/songs/1
```

## Development

### Node.js
- Framework: Express.js
- Port: 3000 (configurable via PORT environment variable)
- Dependencies managed via npm

### Rust
- Framework: Actix-web
- Port: 8080 (configurable via PORT environment variable)
- Dependencies managed via Cargo

## Technologies Used

- **Node.js**: Express.js for the original API
- **Rust**: Actix-web for the ported API
- **Docker**: Containerization for both applications
- **Docker Compose**: Multi-container orchestration
- **GitHub Codespaces**: Cloud development environment

## License

MIT

