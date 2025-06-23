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
