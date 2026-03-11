# ⛽ Gas Tracker

A Rust CLI tool that fetches real-time Ethereum gas prices from the Etherscan API.

## Features

- Fetches **Fast**, **Standard**, and **Safe** gas prices in Gwei
- Displays the latest block number
- Uses Etherscan's Gas Oracle API

## Prerequisites

- Rust (stable)
- An [Etherscan API Key](https://etherscan.io/apis) (free tier works)

## Setup

```bash
cp .env.example .env
# Edit .env and add your Etherscan API key
```

## Usage

```bash
cargo run
```

### Sample Output

```
Fetching current gas prices from Etherscan...
🚀 Fast Gas Price: 25 gwei
🚶 Propose Gas Price: 20 gwei
🐢 Safe Gas Price: 15 gwei
📦 Last Block: 19432156
```

## Tech Stack

- `reqwest` — HTTP client
- `serde` — JSON deserialization
- `dotenv` — Environment variable management
