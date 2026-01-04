import axios from "axios";

const BASE_URL = "/api/v1/accounts";

export const getAccounts = async () => {
  const response = await axios.get(BASE_URL);
  return response.data;
};

export const getAccount = async (id) => {
  const response = await axios.get(`${BASE_URL}/${id}`);
  return response.data;
};

export const getAccountBatches = async (accountId) => {
  const response = await axios.get(`${BASE_URL}/${accountId}/batches`);
  return response.data;
};

export const createAccount = async (account) => {
  const response = await axios.post(BASE_URL, account);
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
  const response = await axios.patch(
    `${BASE_URL}/${accountId}/batches/${batchId}/bets/${betId}`,
    { status }
  );
  return response.data;
};

export const submitBatch = async (accountId, batchId) => {
  const response = await axios.delete(`${BASE_URL}/${accountId}/batches/${batchId}`);
  return response.data;
};

export const subscribeToAccountEvents = (
  onAccountCreated,
  onAccountDeleted,
  onBatchCreated,
  onPing,
  onBetStatusUpdated
) => {
  const SSE_URL = "/sse";
  const eventSource = new EventSource(SSE_URL);

  eventSource.onopen = () => {
    console.log("✅ Connected to SSE stream");
  };

  eventSource.onerror = (err) => {
    console.error("❌ SSE error:", err);
  };

  eventSource.addEventListener("account_created", (event) => {
    try {
      const payload = JSON.parse(event.data);
      onAccountCreated?.(payload.account);
    } catch (err) {
      console.error("Failed to parse account_created event:", err);
    }
  });

  eventSource.addEventListener("account_deleted", (event) => {
    try {
      const payload = JSON.parse(event.data);
      onAccountDeleted?.(payload.id);
    } catch (err) {
      console.error("Failed to parse account_deleted event:", err);
    }
  });

  eventSource.addEventListener("batch_created", (event) => {
    try {
      const payload = JSON.parse(event.data);
      onBatchCreated?.(payload.batch);
    } catch (err) {
      console.error("Failed to parse batch_created event:", err);
    }
  });

  eventSource.addEventListener("batch_completed", (event) => {
    try {
      const payload = JSON.parse(event.data);
      // Mark as completed for filtering
      onBatchCreated?.({ ...payload, completed: true });
    } catch (err) {
      console.error("Failed to parse batch_completed event:", err);
    }
  });

  eventSource.addEventListener("bet_status_updated", (event) => {
    try {
      const payload = JSON.parse(event.data);
      onBetStatusUpdated?.(payload.bet);
    } catch (err) {
      console.error("Failed to parse bet_status_updated event:", err);
    }
  });

  eventSource.addEventListener("ping", (event) => {
    onPing?.(event.data);
  });

  return eventSource;
};
