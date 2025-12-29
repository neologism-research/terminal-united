#!/usr/bin/env node
const { spawn } = require("child_process");
const path = require("path");
const fs = require("fs");

// Path to the binary
const binaryPath = path.join(__dirname, "bin", "terminal-united");

// Check if the binary exists and is executable
if (!fs.existsSync(binaryPath)) {
  console.error(`Error: Binary not found at ${binaryPath}`);
  process.exit(1);
}

// Set executable permissions (important for Linux/macOS)
try {
  fs.chmodSync(binaryPath, 0o755);
} catch (e) {
  // Ignore errors if we can't chmod (e.g. on Windows)
}

// Launch the game
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: "inherit", // This is vital for Ratatui to capture keyboard input
  windowsHide: false,
});

child.on("exit", (code) => {
  process.exit(code || 0);
});

child.on("error", (err) => {
  console.error("Failed to start Terminal United:", err);
  process.exit(1);
});
