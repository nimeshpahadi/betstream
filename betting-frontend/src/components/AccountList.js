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
      const data = JSON.parse(event.data);
      const { account_id, account_name, account_hostname } = data;

      // Add new account if not already present
      setAccounts((prev) => {
        const exists = prev.some(acc => acc.id === account_id);
        if (exists) return prev;
        return [
          ...prev,
          {
            id: account_id,
            name: account_name,
            hostname: account_hostname,
            created_at: "",   // Or set actual value if available
            updated_at: "",   // Or set actual value if available
          },
        ];
      });
    } catch (err) {
      console.error("Failed to parse account_created SSE data", err);
    }
  });

  eventSource.addEventListener("account_deleted", (event) => {
    try {
      const data = JSON.parse(event.data);
      const { account_id } = data;

      // Remove the deleted account
      setAccounts((prev) => prev.filter(acc => acc.id !== account_id));
    } catch (err) {
      console.error("Failed to parse account_deleted SSE data", err);
    }
  });

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
