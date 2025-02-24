# personal_finance_tracker

**A full‐stack Rust (Rocket) application for managing your finances.**

## Features
- **Add Transactions** (income or expense), specifying:
  - `date` (YYYY‐MM‐DD)
  - `category`
  - `description`
  - `amount` (+ for income, − for expense)
- **List / Filter** by date range or category  
- **Edit & Delete** any transaction by ID  
- **Summaries** showing total balance, total income, and total expenses  
- **Local JSON Storage** (`data.json`) for simplicity

## Project Structure
personal_finance_tracker/ ├── Cargo.toml ├── Rocket.toml ├── src │ └── main.rs # Rocket routes, in-memory state, JSON reading/writing └── static ├── index.html # Front-end layout ├── main.js # JavaScript to call /api routes └── style.css # Optional styling

## Usage

- **Add** a transaction via the form on `index.html`
- **View** transactions in the table
- **Edit/Delete** by clicking relevant buttons (if implemented)
- **Filter** by date or category (front-end form or query parameters)
- **Check Summaries** with `/api/summary` or front-end summary button

## Future Enhancements

- Database integration (SQLite, Postgres)
- Authentication for multi-user support
- Charts/Visualizations for monthly spending
- UI Framework (React/Vue/Angular) for a richer interface

