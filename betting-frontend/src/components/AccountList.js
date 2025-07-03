import { useState, useEffect, useRef } from 'react';
import {
  User, Calendar, Clock, DollarSign, Target, Trophy, AlertCircle, CheckCircle, XCircle
} from 'lucide-react';
import {
  getAccounts, getAccount, getAccountBatches, subscribeToAccountEvents,
  updateBetStatus, submitBatch, cancelBatch
} from '../api/accounts';

function BetStatusSelector({ bet, onChange }) {
  return (
    <div className="flex justify-center items-center space-x-4">
      <button
        onClick={() => onChange('successful')}
        title="Mark as Successful"
        className={`hover:text-green-400 ${bet.status === 'successful' ? 'text-green-500' : 'text-gray-400'}`}
      >
        <CheckCircle className="w-6 h-6" />
      </button>
      <button
        onClick={() => onChange('failed')}
        title="Mark as Failed"
        className={`hover:text-red-400 ${bet.status === 'failed' ? 'text-red-500' : 'text-gray-400'}`}
      >
        <XCircle className="w-6 h-6" />
      </button>
    </div>
  );
}

export default function AccountBatchesUI() {
  const [accounts, setAccounts] = useState([]);
  const [accountId, setAccountId] = useState(null);
  const [batches, setBatches] = useState([]);
  const [account, setAccount] = useState(null);
  const [loading, setLoading] = useState(true);
  const [selectedBatchId, setSelectedBatchId] = useState(null);
  const [error, setError] = useState(null);
  const accountIdRef = useRef(accountId);

  const selectedBatch = batches.find(b => b.id === selectedBatchId);

  useEffect(() => {
    accountIdRef.current = accountId;
  }, [accountId]);

  useEffect(() => {
    const loadInitialAccounts = async () => {
      try {
        const data = await getAccounts();
        setAccounts(data);
        if (data.length > 0) {
          setAccountId(data[0].id);
        }
      } catch (err) {
        setError('Failed to load accounts');
      }
    };
    loadInitialAccounts();

    const es = subscribeToAccountEvents(
      (newAccount) => {
        setAccounts((prev) => {
          const exists = prev.some(acc => acc.id === newAccount.id);
          const updated = exists ? prev : [newAccount, ...prev];
          return updated;
        });
        setAccountId(newAccount.id);
      },
      (deletedId) => {
        setAccounts((prev) => {
          const filtered = prev.filter(acc => acc.id !== deletedId);
          setAccountId((prevId) =>
            prevId === deletedId ? (filtered[0]?.id ?? null) : prevId
          );
          return filtered;
        });
      },
      (batchAccountId) => {
        if (batchAccountId === accountIdRef.current) {
          getAccountBatches(accountIdRef.current)
            .then((batchesData) => {
              const activeBatches = batchesData.filter(batch => !batch.completed);
              setBatches(activeBatches);
              setSelectedBatchId(activeBatches[0]?.id || null);
            })
            .catch((err) => {
              console.error("Failed to reload batches after batch_created", err);
            });
        }
      },
      (ping) => console.log("💓 Ping:", ping),
      (updatedBet) => {
        setBatches(prev =>
          prev.map(batch =>
            batch.id === updatedBet.batch_id
              ? {
                  ...batch,
                  bets: batch.bets.map(bet =>
                    bet.pid === updatedBet.pid ? { ...bet, status: updatedBet.status } : bet
                  )
                }
              : batch
          )
        );
      }
    );

    return () => es.close();
  }, []);

  useEffect(() => {
    if (!accountId) return;

    const fetchData = async () => {
      setLoading(true);
      setError(null);
      try {
        const [accountData, batchesData] = await Promise.all([
          getAccount(accountId),
          getAccountBatches(accountId),
        ]);
        // Filter out completed batches
        const activeBatches = batchesData.filter(batch => !batch.completed);
        setAccount(accountData);
        setBatches(activeBatches);
        setSelectedBatchId(activeBatches[0]?.id || null);
      } catch (err) {
        console.error("Data fetch error:", err);
        setError("Failed to load account and batch data");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [accountId]);

  const formatDate = (dateString) => new Date(dateString).toLocaleString("en-US");
  const calculateTotalStake = (bets) =>
    bets.reduce((sum, bet) => sum + (bet.stake || 0), 0).toFixed(2);

  const handleStatusChange = async (betId, status) => {
    if (!selectedBatch || !accountId) return;

    try {
      const updatedBet = await updateBetStatus(accountId, selectedBatch.id, betId, status);
      setBatches(prev =>
        prev.map(batch =>
          batch.id === updatedBet.batch_id
            ? {
                ...batch,
                bets: batch.bets.map(bet =>
                  bet.pid === updatedBet.pid ? { ...bet, status: updatedBet.status } : bet
                )
              }
            : batch
        )
      );
    } catch (err) {
      console.error("Failed to update bet status", err);
    }
  };

  const handleSubmitBatch = async () => {
    if (!selectedBatch || !accountId) return;

    try {
      await submitBatch(accountId, selectedBatch.id);

      // Remove the batch from local UI state
      setBatches((prev) => prev.filter(batch => batch.id !== selectedBatch.id));

      // Clear the selected batch
      setSelectedBatchId(null);
    } catch (err) {
      console.error("Failed to submit batch", err);
    }
  };

  const handleCancelBatch = () => {
    if (!selectedBatch || !accountId) return;
    cancelBatch(accountId, selectedBatch.id);
  };

  if (loading) {
    return <div className="min-h-screen bg-gray-900 text-white flex justify-center items-center">Loading...</div>;
  }

  if (error) {
    return <div className="min-h-screen bg-gray-900 text-red-400 flex justify-center items-center">{error}</div>;
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white p-4 grid grid-cols-1 lg:grid-cols-4 gap-6">
      <aside className="bg-gray-800 border border-gray-700 p-4 rounded-xl">
        <h2 className="text-sm text-gray-300 mb-2">Accounts</h2>
        <ul className="space-y-2 mb-4">
          {accounts.map((acc) => (
            <li key={acc.id}>
              <button
                className={`w-full text-left p-2 rounded-md border text-sm font-medium ${
                  accountId === acc.id
                    ? "bg-blue-900/50 border-blue-400 text-blue-200"
                    : "border-gray-600 text-gray-300 hover:bg-gray-700/40"
                }`}
                onClick={() => setAccountId(acc.id)}
              >
                {acc.name || `Account ${acc.id}`}
              </button>
            </li>
          ))}
        </ul>

        {account && (
          <div className="space-y-2 text-sm text-gray-300">
            <p><User className="inline w-4 h-4 mr-1" /> {account.name || 'Unnamed'}</p>
            <p>Total Batches: {batches.length}</p>
            <p>Active: {batches.filter(b => !b.completed).length}</p>
            <p>Completed: {batches.filter(b => b.completed).length}</p>

            <h3 className="text-sm text-gray-300 mt-4 mb-1">Select Batch</h3>
            <ul className="space-y-2">
              {batches.map((batch) => (
                <li key={batch.id}>
                  <button
                    className={`w-full text-left p-2 rounded-md border text-sm ${
                      selectedBatchId === batch.id
                        ? "bg-blue-900/50 border-blue-400 text-blue-200"
                        : "border-gray-600 text-gray-300 hover:bg-gray-700/50"
                    }`}
                    onClick={() => setSelectedBatchId(batch.id)}
                  >
                    {batch.meta?.name || `Batch ${batch.id}`}
                  </button>
                </li>
              ))}
            </ul>
          </div>
        )}
      </aside>

      <section className="lg:col-span-3 space-y-4">
        {selectedBatch && (
          <div className="space-y-4">
            <div className="bg-gray-800 border border-gray-700 rounded-xl p-4">
              <div className="flex justify-between items-center mb-4">
                <h1 className="text-lg font-bold">{selectedBatch.meta?.name || `Batch ${selectedBatch.id}`}</h1>
                <span className={`px-3 py-1 text-sm rounded-full border ${
                  selectedBatch.completed
                    ? "text-green-300 border-green-600 bg-green-900/20"
                    : "text-orange-300 border-orange-600 bg-orange-900/20"
                }`}>
                  {selectedBatch.completed ? "Completed" : "Active"}
                </span>
              </div>
              <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 text-sm text-gray-300">
                <div><Calendar className="inline w-4 h-4 mr-1" /> {formatDate(selectedBatch.created_at)}</div>
                <div><Target className="inline w-4 h-4 mr-1" /> Total Bets: {selectedBatch.bets?.length || 0}</div>
                <div><DollarSign className="inline w-4 h-4 mr-1" /> Total Stake: ${calculateTotalStake(selectedBatch.bets)}</div>
              </div>
            </div>

            <div className="bg-gray-800 border border-gray-700 rounded-xl overflow-x-auto">
              <table className="min-w-full text-sm text-center">
                <thead className="bg-gray-700 text-gray-300">
                  <tr>
                    <th className="px-4 py-2">#</th>
                    <th className="px-4 py-2">Selection</th>
                    <th className="px-4 py-2">Stake</th>
                    <th className="px-4 py-2">Cost</th>
                    <th className="px-4 py-2">Status</th>
                    <th className="px-4 py-2">Update Status</th>
                  </tr>
                </thead>
                <tbody>
                  {selectedBatch.bets?.map((bet) => (
                    <tr
                      key={bet.pid}
                      className={`border-t border-gray-700 ${bet.status === 'successful'
                          ? 'bg-green-900/20'
                          : bet.status === 'failed'
                            ? 'bg-red-900/20'
                            : ''
                        }`}
                    >
                      <td className="px-4 py-2">{bet.id}</td>
                      <td className="px-4 py-2">{bet.selection}</td>
                      <td className="px-4 py-2">${bet.stake}</td>
                      <td className="px-4 py-2">${bet.cost}</td>
                      <td className="px-4 py-2 capitalize">{bet.status}</td>
                      <td className="px-4 py-2">
                        <BetStatusSelector
                          bet={bet}
                          onChange={(newStatus) => handleStatusChange(bet.pid, newStatus)}
                        />
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            <div className="flex justify-end space-x-4">
              <button onClick={handleSubmitBatch} className="bg-green-700 text-white px-2 py-1 rounded-md hover:bg-green-800">
                <CheckCircle className="inline w-4 h-4 mr-1" /> Submit
              </button>
              <button onClick={handleCancelBatch} className="bg-red-700 text-white px-2 py-1 rounded-md hover:bg-red-800">
                <XCircle className="inline w-4 h-4 mr-1" /> Cancel
              </button>
            </div>
          </div>
        )}
      </section>
    </div>
  );
}
