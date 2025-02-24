# personal_finance_tracker
**Features**
Add Transactions: Enter date, category, description, amount (+ for income, − for expense).
List/Filter: Query transactions with optional start/end dates and category filters.
Edit & Delete: Update any field or remove a transaction by ID.
Summaries: Get total balance, total income, expense, and transaction count (optionally filtered by year/month).
Local JSON Storage: Data persists in data.json for simplicity (a database can be used later).
**Project Structure**
personal_finance_tracker/
├── Cargo.toml
├── Rocket.toml            # Optional Rocket config (port, address)
├── src
│   └── main.rs            # Rocket routes, data logic, in-memory state
└── static
    ├── index.html         # Basic front-end layout
    ├── main.js            # Fetch calls to /api endpoints, form & table logic
    └── style.css          # Optional styling
**Future Enhancements**
Database Integration: Move from data.json to a real DB (SQLite, Postgres, etc.).
Authentication: Support multiple users, secure endpoints.
Charts/Visualizations: Graph monthly spending or categories.
UI Framework: Replace the basic front end with a modern JS framework (React, Vue, etc.).
