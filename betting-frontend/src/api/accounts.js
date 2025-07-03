import axios from "axios";

const BASE_URL = "/api/v1/accounts";

export const getAccounts = async () => {
  const response = await axios.get(BASE_URL);
  return response.data;
};

export const getAccountBatches = async (accountId) => {
  const res = await fetch(`${BASE_URL}/${accountId}/batches`);
  if (!res.ok) throw new Error(`Failed to fetch batches`);
  return await res.json();
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

export const updateBetStatus = async (accountId, batchId, betId, status) => {
  const response = await axios.patch(`${BASE_URL}/${accountId}/batches/${batchId}/bets/${betId}`, { status });
  return response.data;
};

export const submitBatch = async (accountId, batchId) => {
  const response = await axios.delete(`${BASE_URL}/${accountId}/batches/${batchId}`);
  return response.data;
};

// export const submitBatch = async (accountId, batchId, bets) => {
//   const response = await axios.delete(`${BASE_URL}/${accountId}/batches/${batchId}/bets`, bets);
//   return response.data;
// };

export const cancelBatch = async (accountId, batchId, bets) => {
  const response = await axios.patch(`${BASE_URL}/${accountId}/batches/${batchId}/bets`, bets);
  return response.data;
};

export const subscribeToAccountEvents = (
  onAccountCreated,
  onAccountDeleted,
  onBatchCreated,
  onPing,
  onBetStatusUpdated
) => {
  const eventSource = new EventSource("http://localhost:3001/sse");

  eventSource.onopen = () => {
    console.log("✅ Connected to SSE stream");
  };

  eventSource.onerror = (err) => {
    console.error("❌ SSE error:", err);
  };

  eventSource.addEventListener("account_created", (event) => {
    try {
      const account = JSON.parse(event.data);
      if (onAccountCreated) {
        onAccountCreated(account);
      }
    } catch (err) {
      console.error("Failed to parse account_created event:", err);
    }
  });

  eventSource.addEventListener("account_deleted", (event) => {
    try {
      const payload = JSON.parse(event.data);
      const deletedId = payload.id ?? payload.pk;
      if (deletedId && onAccountDeleted) {
        onAccountDeleted(deletedId);
      }
    } catch (err) {
      console.error("Failed to parse account_deleted event:", err);
    }
  });

  eventSource.addEventListener("batch_created", (event) => {
    const data = JSON.parse(event.data);
    if (onBatchCreated) onBatchCreated(data.id);
  });

  eventSource.addEventListener("bet_status_updated", (event) => {
    try {
      const data = JSON.parse(event.data);
      if (onBetStatusUpdated) onBetStatusUpdated(data);
    } catch (err) {
      console.error("Failed to parse bet_status_updated event:", err);
    }
  });

  eventSource.addEventListener("ping", (event) => {
    if (onPing) {
      onPing(event.data);
    }
  });

  return eventSource;
};
