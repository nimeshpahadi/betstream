-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    hostname TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_accounts_name ON accounts(name);
CREATE INDEX IF NOT EXISTS idx_accounts_hostname ON accounts(hostname);
CREATE INDEX IF NOT EXISTS idx_accounts_created_at ON accounts(created_at);


CREATE TABLE IF NOT EXISTS batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    completed BOOLEAN NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
    created_at DATETIME DEFAULT (datetime('now')),
    updated_at DATETIME DEFAULT (datetime('now')),
    meta TEXT NOT NULL,
    account_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bets (
    id INTEGER PRIMARY KEY,
    selection TEXT NOT NULL,
    stake REAL NOT NULL,
    cost REAL NOT NULL,
    batch_id INTEGER NOT NULL,
    FOREIGN KEY (batch_id) REFERENCES batches(id) ON DELETE CASCADE
);

CREATE TRIGGER IF NOT EXISTS batches_update
AFTER UPDATE ON batches
BEGIN
    UPDATE batches SET updated_at = datetime('now') WHERE id = NEW.id;
END;