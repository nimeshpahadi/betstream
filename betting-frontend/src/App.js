import React from "react";
import AccountList from "./components/AccountList";
import AccountSSEListener from "./components/AccountSSEListener"

function App() {
  return (
    <div style={{
      backgroundColor: "#121212",
      color: "#ffffff",
      minHeight: "100vh",
      padding: "2rem",
      fontFamily: "sans-serif"
    }}>
      <h1 style={{ fontSize: "2rem", marginBottom: "1rem" }}>🎲 Betting Accounts</h1>
      <AccountList />
    </div>
  );
}

export default App;
