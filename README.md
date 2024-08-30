# Macro Dashboard ACM

This project is a Rust-based backend service designed to provide macroeconomic data through various API endpoints, such as current inflation rates, T-bill rates, and real T-bill yields. The backend is implemented using the Warp web framework and includes logging capabilities using `env_logger` and `log`.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Node.js and npm](https://nodejs.org/) (for the frontend)
- [React](https://reactjs.org/)

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/macro_dashboard_acm.git
cd macro_dashboard_acm
```

### 2. Setup the Backend

#### a. Install Rust Dependencies

```bash
cd backend
cargo build
```

#### b. Configure Environment Variables

To run this project, you need to create a `.env` file in the `backend/` directory with your own BLS API key:

```bash
BLS_API_KEY=your_api_key_here
```

#### c. Run the Backend Server

Start the backend server:

```bash
cargo run
```

The backend server will start at `http://127.0.0.1:3030`.

### 3. Setup the Frontend

#### a. Install Node.js Dependencies

```bash
cd ../frontend
npm install
```

#### b. Start the Frontend Server

```bash
npm start
```

The frontend will start at `http://localhost:3000` and will communicate with the backend server.

## Logging

### Using `env_logger` and `log`

The backend uses the `env_logger` and `log` crates for logging. This allows you to control the level of logging output via environment variables.

### Enabling Logging

To enable logging, set the `RUST_LOG` environment variable when running the backend:

```bash
RUST_LOG=info cargo run
```

### Log Levels

- `trace`: Trace-level logging, the most verbose level.
- `debug`: Debug-level logging, useful for development.
- `info`: Information-level logging, suitable for general information.
- `warn`: Warning-level logging, for non-critical issues.
- `error`: Error-level logging, for critical errors.

### Example: Running the Backend with Debug Logging

```bash
RUST_LOG=debug cargo run
```

This will enable debug-level logging, which provides detailed information about the backend's operations.

### Disabling Logging

If you want to disable logging, you can simply omit the `RUST_LOG` environment variable or set it to a higher threshold that excludes other levels:

```bash
RUST_LOG=off cargo run
```

### Viewing Logs

Logs will be output to the console where you run the `cargo run` command. You can redirect these logs to a file if needed:

```bash
RUST_LOG=info cargo run > backend.log 2>&1
```

This command will write the log output to `backend.log`.

## Troubleshooting

- **CORS Issues**: Ensure the backend is configured to allow cross-origin requests from the frontend, especially if they run on different ports (e.g., `localhost:3030` for the backend and `localhost:3000` for the frontend).
- **API Errors**: Check the backend logs for any errors related to API calls or data processing.
- **Frontend Issues**: Use the browser's developer tools to inspect network requests and logs.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Feel free to submit issues and pull requests. Contributions are welcome!