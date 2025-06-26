// const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3000';
const API_BASE_URL = "/api/v1";

export const createBatch = async (accountId, payload) => {
  const response = await fetch(`${API_BASE_URL}/accounts/${accountId}/batches`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new Error(`Failed to create batch: ${response.status} ${response.statusText}`);
  }

  return response.json();
};

export const getBatch = async (accountId, batchId) => {
  const response = await fetch(`${API_BASE_URL}/accounts/${accountId}/batches/${batchId}`);

  if (!response.ok) {
    throw new Error(`Failed to get batch: ${response.status} ${response.statusText}`);
  }

  return response.json();
};

export const getBatches = async (accountId) => {
  const response = await fetch(`${API_BASE_URL}/accounts/${accountId}/batches`);

  if (!response.ok) {
    throw new Error(`Failed to get batches: ${response.status} ${response.statusText}`);
  }

  return response.json();
};

export const updateBatch = async (accountId, batchId, payload) => {
  const response = await fetch(`${API_BASE_URL}/accounts/${accountId}/batches/${batchId}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new Error(`Failed to update batch: ${response.status} ${response.statusText}`);
  }

  return response.json();
};

export const deleteBatch = async (accountId, batchId) => {
  const response = await fetch(`${API_BASE_URL}/accounts/${accountId}/batches/${batchId}`, {
    method: 'DELETE',
  });

  if (!response.ok) {
    throw new Error(`Failed to delete batch: ${response.status} ${response.statusText}`);
  }
};