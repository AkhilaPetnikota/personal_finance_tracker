#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::{Json, Value, json};
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDate, Datelike};

use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

static DATA_FILE: &str = "data.json";

/// Global state: transactions stored in-memory behind a Mutex.
struct AppState {
    transactions: Mutex<Vec<Transaction>>,
}

/// Our transaction model.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Transaction {
    id: Uuid,
    date: NaiveDate,
    category: String,
    description: String,
    amount: f64,
}

/// For creating a new transaction (no ID).
#[derive(Debug, Deserialize)]
struct NewTransaction {
    date: String,
    category: String,
    description: String,
    amount: f64,
}

/// For updating an existing transaction (optional fields).
#[derive(Debug, Deserialize)]
struct UpdateTransaction {
    date: Option<String>,
    category: Option<String>,
    description: Option<String>,
    amount: Option<f64>,
}

impl Transaction {
    fn new(date: NaiveDate, category: String, description: String, amount: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            date,
            category,
            description,
            amount,
        }
    }
}

/// Load transactions from a local JSON file (data.json).
fn load_from_file() -> Vec<Transaction> {
    let mut file = match OpenOptions::new()
        .read(true)
        .create(true)
        .open(DATA_FILE)
    {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Could not open data file.");
            return Vec::new();
        }
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        eprintln!("Could not read data file.");
        return Vec::new();
    }

    if contents.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).unwrap_or_else(|_| {
            eprintln!("Error parsing data. Starting with an empty list.");
            Vec::new()
        })
    }
}

/// Save transactions to the JSON file.
fn save_to_file(transactions: &Vec<Transaction>) {
    let json_data = match serde_json::to_string_pretty(transactions) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Could not convert transactions to JSON.");
            return;
        }
    };

    if let Ok(mut file) = File::create(DATA_FILE) {
        let _ = file.write_all(json_data.as_bytes());
    } else {
        eprintln!("Could not create data file.");
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load data at startup
    let initial_data = load_from_file();
    let state = AppState {
        transactions: Mutex::new(initial_data),
    };

    // Build & launch Rocket
    rocket::build()
        .manage(state)
        // Serve all static files under "static/" at the root path
        .mount("/", FileServer::from(relative!("static")))
        // Expose API routes under /api
        .mount("/api", routes![
            get_transactions,
            post_transaction,
            update_transaction,
            delete_transaction,
            get_summary
        ])
        .launch()
        .await?;

    Ok(())
}

/// GET /api/transactions?<start_date>&<end_date>&<category>
/// Example: GET /api/transactions?start_date=2025-05-01&end_date=2025-05-31&category=Food
#[get("/transactions?<start_date>&<end_date>&<category>")]
async fn get_transactions(
    state: &State<AppState>,
    start_date: Option<String>,
    end_date: Option<String>,
    category: Option<String>,
) -> Json<Vec<Transaction>> {
    let transactions = state.transactions.lock().unwrap();

    // Filter the list based on the optional query parameters
    let filtered: Vec<Transaction> = transactions
        .iter()
        .cloned()
        .filter(|tx| {
            // Filter by category
            if let Some(ref cat) = category {
                // If user provided a non-empty category, compare case-insensitively
                if !cat.is_empty() && tx.category.to_lowercase() != cat.to_lowercase() {
                    return false;
                }
            }
            // Filter by start_date
            if let Some(ref sd) = start_date {
                if let Ok(parsed_sd) = NaiveDate::parse_from_str(sd, "%Y-%m-%d") {
                    if tx.date < parsed_sd {
                        return false;
                    }
                }
            }
            // Filter by end_date
            if let Some(ref ed) = end_date {
                if let Ok(parsed_ed) = NaiveDate::parse_from_str(ed, "%Y-%m-%d") {
                    if tx.date > parsed_ed {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

    Json(filtered)
}

/// POST /api/transactions
/// Create a new transaction from JSON body.
#[post("/transactions", format = "application/json", data = "<new_tx>")]
async fn post_transaction(
    state: &State<AppState>,
    new_tx: Json<NewTransaction>,
) -> Value {
    let mut transactions = state.transactions.lock().unwrap();

    // Parse the date
    let date_parsed = match NaiveDate::parse_from_str(&new_tx.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return json!({"error": "Invalid date format. Use YYYY-MM-DD."}),
    };

    // Create and add new transaction
    let tx = Transaction::new(
        date_parsed,
        new_tx.category.clone(),
        new_tx.description.clone(),
        new_tx.amount,
    );
    transactions.push(tx.clone());

    // Save
    save_to_file(&transactions);

    json!({ "status": "success", "transaction": tx })
}

/// PUT /api/transactions/<id>
/// Update an existing transaction by ID.
#[put("/transactions/<id>", format = "application/json", data = "<update_data>")]
async fn update_transaction(
    state: &State<AppState>,
    id: &str,
    update_data: Json<UpdateTransaction>,
) -> Value {
    let mut transactions = state.transactions.lock().unwrap();

    // Parse the ID (UUID)
    let parsed_id = match Uuid::parse_str(id) {
        Ok(uid) => uid,
        Err(_) => return json!({"error": "Invalid UUID."}),
    };

    // Find the transaction's index in the vector
    if let Some(idx) = transactions.iter().position(|t| t.id == parsed_id) {
        // We only hold a mutable reference to this single element
        {
            let tx = &mut transactions[idx];

            // Update date if provided
            if let Some(ref date_str) = update_data.date {
                let parsed_date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    Ok(d) => d,
                    Err(_) => return json!({ "error": "Invalid date format (YYYY-MM-DD)." }),
                };
                tx.date = parsed_date;
            }
            // Update category if provided
            if let Some(ref cat) = update_data.category {
                tx.category = cat.clone();
            }
            // Update description if provided
            if let Some(ref desc) = update_data.description {
                tx.description = desc.clone();
            }
            // Update amount if provided
            if let Some(am) = update_data.amount {
                tx.amount = am;
            }
        }
        // Now that we're done mutably borrowing the element, we can save
        save_to_file(&transactions);

        let updated_tx = &transactions[idx];
        return json!({ "status": "success", "transaction": updated_tx });
    } else {
        return json!({ "error": "Transaction not found." });
    }
}

/// DELETE /api/transactions/<id>
/// Remove the transaction with the given UUID.
#[delete("/transactions/<id>")]
async fn delete_transaction(state: &State<AppState>, id: &str) -> Value {
    let mut transactions = state.transactions.lock().unwrap();

    let parsed_id = match Uuid::parse_str(id) {
        Ok(uid) => uid,
        Err(_) => return json!({"error": "Invalid UUID."}),
    };

    let before_len = transactions.len();
    // Retain all except the one with parsed_id
    transactions.retain(|t| t.id != parsed_id);

    // If it actually removed something
    if transactions.len() < before_len {
        save_to_file(&transactions);
        json!({ "status": "success" })
    } else {
        json!({ "error": "Transaction not found." })
    }
}

/// GET /api/summary?<year>&<month>
/// Returns total, income, expense for all (or for a given year/month).
#[get("/summary?<year>&<month>")]
async fn get_summary(
    state: &State<AppState>,
    year: Option<String>,
    month: Option<String>,
) -> Value {
    let transactions = state.transactions.lock().unwrap();

    // Filter by year/month if provided
    let mut filtered = Vec::new();
    for tx in transactions.iter() {
        if let Some(ref y_str) = year {
            if let Ok(y) = y_str.parse::<i32>() {
                if tx.date.year() != y {
                    continue;
                }
            }
        }
        if let Some(ref m_str) = month {
            if let Ok(m) = m_str.parse::<u32>() {
                if tx.date.month() != m {
                    continue;
                }
            }
        }
        filtered.push(tx);
    }

    let total: f64 = filtered.iter().map(|t| t.amount).sum();
    let income: f64 = filtered.iter().filter(|t| t.amount > 0.0).map(|t| t.amount).sum();
    let expense: f64 = filtered.iter().filter(|t| t.amount < 0.0).map(|t| t.amount).sum();

    json!({
        "total": total,
        "income": income,
        "expense": expense,
        "transactions_count": filtered.len()
    })
}
