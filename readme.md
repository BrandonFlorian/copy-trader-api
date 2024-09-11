# Copy Trader API

This is a WIP Rust-based implementation of an existing API that I use in one of my private repos for a Solana copy trading system that integrates with Supabase. The bot is not yet present in this repo at this time and the python implementation remains private.

I am still learning Rust (and using this as a way to do so!) but eventually I do plan to convert the bot as well.

In the python server, to prevent a private key from being exposed, the wallet secret key is set in the .env file and then we derive the address from that. In this Rust version, we are not yet doing any logic that would require a private key, but we do need the address so it needs to be set in the .env file.

Please note that this is a work in progress and the API will be expanded upon as the repo is converted to Rust.

## Features

- At this time you can only interact with the Supabase db.
- Track wallets
- Manage copy trade settings
- View transaction history
- Integration with Supabase for data storage

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Supabase account and project

## Setup

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/copy-trader-api.git
   cd copy-trader-api
   ```

2. Set up environment variables:
   Create a `.env` file in the project root and add the following:

   ```
   SUPABASE_URL=your_supabase_project_url
   SUPABASE_API_KEY=your_supabase_api_key
   SUPABASE_SERVICE_ROLE_KEY=your_supabase_service_role_key
   USER_ID=default_user_id
   APP_PORT=3001
   ```

3. Install dependencies:
   ```
   cargo build
   ```

## Running the API

To start the API server:

```
cargo run
```

The server will start on `http://0.0.0.0:3001` by default.

## API Endpoints

- `GET /tracked_wallets`: Get all tracked wallets
- `POST /tracked_wallets`: Add a new tracked wallet
- `PUT /tracked_wallets/archive/:wallet_address`: Archive a tracked wallet
- `PUT /tracked_wallets/unarchive/:wallet_address`: Unarchive a tracked wallet
- `DELETE /tracked_wallets/:wallet_address`: Delete a tracked wallet
- `PUT /tracked_wallets/update`: Update a tracked wallet
- `GET /copy_trade_settings`: Get copy trade settings
- `POST /copy_trade_settings`: Create new copy trade settings
- `PUT /copy_trade_settings`: Update copy trade settings
- `DELETE /copy_trade_settings/:tracked_wallet_id`: Delete copy trade settings
- `GET /transaction_history`: Get transaction history

## Development

To run tests:

```
cargo test
```

To check for linting issues:

```
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
