import React, { useEffect, useState } from "react";

export default function AccountSSEListener() {
  const [accounts, setAccounts] = useState([]);

  useEffect(() => {
    const eventSource = new EventSource("http://localhost:3001/sse");

    eventSource.addEventListener("account_created", (event) => {
      try {
        const account = JSON.parse(event.data);
        setAccounts(prev => [...prev, account]);
        console.log("New account created:", account);
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
    <div>
      <h2>Accounts Created (live):</h2>
      <ul>
        {accounts.map(acc => (
          <li key={acc.pk || acc.id || acc.account_id}>
            ID: {acc.id} — Account ID: {acc.account_id} — Event: {acc.event}
          </li>
        ))}
      </ul>
    </div>
  );
}
