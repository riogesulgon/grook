# Grook 🚀

**Lightning-Fast AI CLI Tool** ⚡

A high-performance Rust CLI tool for interacting with Groq's API using the Mixtral 8x7b model.

✨ **Key Features:**
- 🏎️ **Blazing Fast**: Combines Rust's performance with Groq's cutting-edge AI acceleration
- 🧠 **Powerful AI**: Leverages Groq's Mixtral 8x7b model for advanced AI interactions
- 🌈 **Rich Output**: Beautiful markdown formatting with syntax highlighting
- ⚡ **Instant Responses**: Optimized for speed and efficiency

## Installation

### Option 1: Install from source
1. Install Rust from [rustup.rs](https://rustup.rs/) if not already installed
2. Clone this repository:
   ```bash
   git clone https://github.com/riogesulgon/grook.git
   cd grook
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Set up your Groq API key using the setup script:
   ```bash
   chmod +x scripts/init_groq.sh
   ./scripts/init_groq.sh
   ```

### Option 2: Install using Cargo
1. Install Rust from [rustup.rs](https://rustup.rs/) if not already installed
2. Install Grook globally:
   ```bash
   cargo install --git https://github.com/riogesulgon/grook.git
   ```
3. Set up your Groq API key:
   ```bash
   grook --init
   ```

## Usage

Send a message to Groq's API:
```bash
cargo run "Your message here"
```
Or if installed globally:
```bash
grook "Your message here"
```

Example:
```bash
grook "Explain quantum computing in simple terms"
```

For release mode:
```bash
cargo run --release "Your message here"
```

## API Configuration

The following parameters are used in the API request:
- Model: mixtral-8x7b-32768
- Temperature: 0.5
- Max tokens: 1024
- Top-p: 1
- Streaming: false

You can modify these parameters in src/main.rs.

## Development

To run tests:
```bash
cargo test
```

To format code:
```bash
cargo fmt
```

To check for linting issues:
```bash
cargo clippy
```

## Dependencies

- reqwest: HTTP client
- tokio: Async runtime
- serde/serde_json: JSON serialization
- synoptic: Syntax highlighting library
- lliw: Color library for terminal output

## Features

- Markdown-formatted output with syntax highlighting
- Supports various markdown elements like headings, code blocks, links, bold, italic text, and more
- Colorful terminal display with different colors for different markdown tokens