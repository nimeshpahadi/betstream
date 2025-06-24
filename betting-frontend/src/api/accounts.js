import axios from "axios";

const BASE_URL = "/api/v1/accounts";

export const getAccounts = async () => {
  const response = await axios.get(BASE_URL);
  return response.data;
};

export const createAccount = async (account) => {
  const response = await axios.post(BASE_URL, account);
  return response.data;
};

export const getAccount = async (id) => {
  const response = await axios.get(`${BASE_URL}/${id}`);
  return response.data;
};

export const updateAccount = async (id, updatedAccount) => {
  const response = await axios.put(`${BASE_URL}/${id}`, updatedAccount);
  return response.data;
};

export const deleteAccount = async (id) => {
  const response = await axios.delete(`${BASE_URL}/${id}`);
  return response.data;
};

export const subscribeToAccountEvents = (onAccountCreated, onPing) => {
  const eventSource = new EventSource("/sse");

  eventSource.onopen = () => {
    console.log("✅ Connected to SSE stream");
  };

  eventSource.onerror = (err) => {
    console.error("❌ SSE error:", err);
    // You can choose to close or reconnect here
  };

  eventSource.addEventListener("account_created", (event) => {
    const account = JSON.parse(event.data);
    if (onAccountCreated) {
      onAccountCreated(account);
    }
  });

  eventSource.addEventListener("ping", (event) => {
    if (onPing) {
      onPing(event.data);
    }
  });

  return eventSource; // Return so the caller can close it if needed
};
