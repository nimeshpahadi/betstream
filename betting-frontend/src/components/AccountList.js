import React, { useEffect, useState } from "react";
import {
  getAccounts,
  deleteAccount,
  createAccount,
} from "../api/accounts";

function AccountList() {
  const [accounts, setAccounts] = useState([]);
  const [name, setName] = useState("");

  useEffect(() => {
    fetchAccounts();
  }, []);

  const fetchAccounts = async () => {
    const data = await getAccounts();
    setAccounts(data);
  };

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

export default AccountList;
