let editingTransactionId = null;

document.addEventListener("DOMContentLoaded", () => {
  fetchTransactions();

  document.getElementById("add-btn").addEventListener("click", addTransaction);
  document.getElementById("filter-btn").addEventListener("click", fetchTransactions);
  document.getElementById("summary-btn").addEventListener("click", fetchSummary);
  document.getElementById("save-edit-btn").addEventListener("click", saveEdit);
  document.getElementById("cancel-edit-btn").addEventListener("click", hideEditOverlay);
});

// Fetch Transactions with Filters
async function fetchTransactions() {
  const startDate = document.getElementById("start-date").value.trim();
  const endDate = document.getElementById("end-date").value.trim();
  const category = document.getElementById("filter-category").value.trim();

  let url = "/api/transactions";
  const params = new URLSearchParams();

  if (startDate) params.append("start_date", startDate);
  if (endDate) params.append("end_date", endDate);
  if (category) params.append("category", category);

  if (params.toString()) url += "?" + params.toString();

  try {
    const res = await fetch(url);
    const transactions = await res.json();
    populateTable(transactions);
  } catch (error) {
    console.error("Error fetching transactions:", error);
  }
}

// Populate Table
function populateTable(transactions) {
  const tbody = document.querySelector("#transactions-table tbody");
  tbody.innerHTML = "";

  transactions.forEach(({ id, date, category, description, amount }) => {
    const row = document.createElement("tr");

    row.innerHTML = `
      <td>${date}</td>
      <td>${category}</td>
      <td>${description}</td>
      <td>${amount.toFixed(2)}</td>
      <td>
        <button onclick="showEditOverlay(${id}, '${date}', '${category}', '${description}', ${amount})">Edit</button>
        <button style="margin-left: 8px;" onclick="deleteTransaction(${id})">Delete</button>
      </td>
    `;

    tbody.appendChild(row);
  });
}

// Add Transaction
async function addTransaction() {
  const date = document.getElementById("date").value.trim();
  const category = document.getElementById("category").value.trim();
  const description = document.getElementById("description").value.trim();
  const amount = parseFloat(document.getElementById("amount").value.trim());

  if (!date || !category || !description || isNaN(amount)) {
    alert("Please fill out all fields correctly.");
    return;
  }

  try {
    const res = await fetch("/api/transactions", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ date, category, description, amount }),
    });

    const data = await res.json();
    if (data.error) {
      alert("Error: " + data.error);
    } else {
      alert("Transaction added!");
      clearForm("add-form");
      fetchTransactions();
    }
  } catch (error) {
    console.error("Error adding transaction:", error);
  }
}

// Show Edit Overlay
function showEditOverlay(id, date, category, description, amount) {
  editingTransactionId = id;
  document.getElementById("edit-date").value = date;
  document.getElementById("edit-category").value = category;
  document.getElementById("edit-description").value = description;
  document.getElementById("edit-amount").value = amount;
  document.getElementById("edit-overlay").style.display = "block";
}

// Hide Edit Overlay
function hideEditOverlay() {
  editingTransactionId = null;
  document.getElementById("edit-overlay").style.display = "none";
}

// Save Edited Transaction
async function saveEdit() {
  if (!editingTransactionId) return;

  const date = document.getElementById("edit-date").value.trim();
  const category = document.getElementById("edit-category").value.trim();
  const description = document.getElementById("edit-description").value.trim();
  const amount = parseFloat(document.getElementById("edit-amount").value.trim());

  if (!date || !category || !description || isNaN(amount)) {
    alert("Please fill out all fields correctly.");
    return;
  }

  try {
    const res = await fetch(`/api/transactions/${editingTransactionId}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ date, category, description, amount }),
    });

    const data = await res.json();
    if (data.error) {
      alert("Error: " + data.error);
    } else {
      alert("Transaction updated successfully!");
      hideEditOverlay();
      fetchTransactions();
    }
  } catch (error) {
    console.error("Error updating transaction:", error);
  }
}

// Delete Transaction
async function deleteTransaction(id) {
  if (!confirm("Are you sure you want to delete this transaction?")) return;

  try {
    const res = await fetch(`/api/transactions/${id}`, { method: "DELETE" });
    const data = await res.json();

    if (data.error) {
      alert("Error: " + data.error);
    } else {
      alert("Transaction deleted.");
      fetchTransactions();
    }
  } catch (error) {
    console.error("Error deleting transaction:", error);
  }
}

// Fetch Summary
async function fetchSummary() {
  const year = document.getElementById("summary-year").value.trim();
  const month = document.getElementById("summary-month").value.trim();

  let url = "/api/summary";
  const params = new URLSearchParams();
  if (year) params.append("year", year);
  if (month) params.append("month", month);
  if (params.toString()) url += "?" + params.toString();

  try {
    const res = await fetch(url);
    const data = await res.json();

    if (data.error) {
      alert("Error: " + data.error);
    } else {
      document.getElementById("summary-result").innerHTML = `
        <p><strong>Total:</strong> ${data.total.toFixed(2)}</p>
        <p><strong>Income:</strong> ${data.income.toFixed(2)}</p>
        <p><strong>Expense:</strong> ${data.expense.toFixed(2)}</p>
        <p><strong>Number of Transactions:</strong> ${data.transactions_count}</p>
      `;
    }
  } catch (error) {
    console.error("Error getting summary:", error);
  }
}

// Clear Form Fields
function clearForm(formId) {
  document.getElementById(formId).reset();
}
