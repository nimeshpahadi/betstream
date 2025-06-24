import React, { useEffect, useState } from "react";
import { getAccounts } from "../api/accounts";

export default function AccountManager() {
  const [accounts, setAccounts] = useState([]);

  // Fetch initial accounts
  useEffect(() => {
    fetchAccounts();
  }, []);

  const fetchAccounts = async () => {
    const data = await getAccounts();
    setAccounts(data);
  };

  // SSE listener
  useEffect(() => {
    const eventSource = new EventSource("http://localhost:3001/sse");

    eventSource.addEventListener("account_created", (event) => {
      try {
        const account = JSON.parse(event.data);

        // Add new account if not already present
        setAccounts((prev) => {
          const exists = prev.some(acc => acc.id === account.id);
          if (exists) return prev;
          return [...prev, account];
        });

      } catch (err) {
        console.error("Failed to parse SSE data", err);
      }
    });

    eventSource.addEventListener("account_deleted", (event) => {
      try {
        const deletedAccount = JSON.parse(event.data);

        // Remove deleted account from state
        setAccounts((prev) => prev.filter(acc => acc.id !== deletedAccount.id));

      } catch (err) {
        console.error("Failed to parse SSE data", err);
      }
    });

    eventSource.onerror = (error) => {
      console.error("SSE connection error:", error);
      eventSource.close();
    };

    return () => {
      eventSource.close();
    };
  }, []);

  return (
    <div style={{
      backgroundColor: "#1e1e1e",
      padding: "1rem",
      borderRadius: "8px",
      boxShadow: "0 0 10px rgba(0,0,0,0.5)"
    }}>
      <h2 style={{ marginBottom: "1rem" }}>Accounts</h2>
      <ul style={{ listStyle: "none", paddingLeft: 0 }}>
        {accounts.map((acc) => (
          <li
            key={acc.id}
            style={{
              display: "flex",
              justifyContent: "space-between",
              padding: "0.5rem",
              borderBottom: "1px solid #333"
            }}
          >
            <span>{acc.name}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
