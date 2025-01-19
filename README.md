# PhishingGuard Platform

PhishingGuard is a cybersecurity platform with a primary focus on detecting and mitigating phishing threats. The platform also integrates a powerful chatbot developed in **Rust** to provide real-time assistance for managing phishing incidents.

## Key Features

- **Rust-Powered Chatbot**:
  - Real-time responses to user queries.
  - Handles questions such as:
    - "Who clicked on phishing links?"
    - "What are my saved contacts?"
    - "Which routes are available in the application?"
  - Built with the highly efficient **Rust** programming language for speed and reliability.

- **Phishing Detection**:
  - Analyzes links for potential phishing threats.
  - Provides detailed reports and insights.

- **Seamless Integration**:
  - The chatbot is integrated directly into the platform’s user interface.

## Setting Up the Rust Chatbot

### Requirements
- **Rust** (Latest stable version)
- `cargo` (Rust's package manager and build system)

### Installation

1. **Ensure Rust is Installed**:
   - Install Rust using `rustup`:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Confirm the installation:
     ```bash
     rustc --version
     ```

2. **Navigate to the Chatbot Directory**:
   ```bash
   cd chatbot
   ```

3. **Run the Chatbot**:
   ```bash
   cargo run
   ```
   - The chatbot server will start at `http://localhost:5501`.

4. **Testing the Chatbot**:
   - Use a tool like `curl` or Postman to send a request:
     ```bash
     curl -X POST -H "Content-Type: application/json" -d '{"message": "hello"}' http://localhost:3030/chat
     ```
   - You should receive a JSON response like:
     ```json
     {"reply":"Hi there! How can I help you today?"}
     ```

### Chatbot Integration in PhishingGuard
The chatbot serves as the primary interaction point for users, assisting with:
- Phishing activity reports.
- Contact management.
- Navigation routes within the application.

The chatbot is integrated into the Python-based platform through API calls, ensuring seamless communication between the frontend and the Rust backend.

## Rust Chatbot Code Overview
The chatbot is built using:
- **Warp**: A fast and lightweight web framework for Rust.
- **Serde**: For JSON serialization and deserialization.
- **Tokio**: For asynchronous runtime.

Here’s a high-level view of the code structure:
```rust
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    reply: String,
}

#[tokio::main]
async fn main() {
    let chat_route = warp::path("chat")
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: ChatRequest| {
            let reply = match req.message.to_lowercase().as_str() {
                "hello" => "Hi there! How can I help you today?".to_string(),
                "phishing" => "Alice and Bob clicked on phishing links.".to_string(),
                _ => "I didn't understand that. Can you clarify?".to_string(),
            };
            warp::reply::json(&ChatResponse { reply })
        });

    println!("Chatbot running on http://localhost:3030");
    warp::serve(chat_route).run(([127, 0, 0, 1], 3030)).await;
}
```

## Benefits of Using Rust for the Chatbot
- **High Performance**: Rust ensures low-latency responses.
- **Safety**: Memory-safe and eliminates common bugs.
- **Concurrency**: Handles multiple requests seamlessly with Tokio.

## Running the Full Platform
1. Start the Rust chatbot:
   ```bash
   cd chatbot
   cargo run
   ```
2. Start the Python backend:
   ```bash
   python app.py
   ```
3. Access the platform at:
   ```
   http://localhost:5000
   ```

## Contact
For any questions or support:
- **Email**: support@phishingguard.com
- **GitHub Issues**: [Open an Issue](https://github.com/berkhouchedyhia/pishinguard_chatbot).
