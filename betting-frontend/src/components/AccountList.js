import { useState, useEffect } from 'react';
import { User, Calendar, Clock, DollarSign, Target, Trophy, AlertCircle } from 'lucide-react';
import { getAccount, getAccountBatches } from '../api/accounts';

export default function AccountBatchesUI({ accountId = 1 }) {
  const [batches, setBatches] = useState([]);
  const [account, setAccount] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [selectedBatch, setSelectedBatch] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        setError(null);
        const [accountData, batchesData] = await Promise.all([
          getAccount(accountId),
          getAccountBatches(accountId)
        ]);
        setAccount(accountData);
        setBatches(batchesData);
        if (batchesData.length > 0) setSelectedBatch(batchesData[0]);
      } catch (err) {
        console.error(err);
        setError('Failed to load account and batch data');
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [accountId]);

  const formatDate = (dateString) => new Date(dateString).toLocaleString('en-US');

  const calculateTotalStake = (bets) =>
    bets.reduce((sum, bet) => sum + (bet.stake || 0), 0).toFixed(2);

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center text-white">
        Loading...
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center text-red-400">
        <AlertCircle className="mr-2" /> {error}
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white p-6">
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">

        {/* Sidebar */}
        <aside className="bg-gray-800 p-4 rounded-xl border border-gray-700">
          <div className="flex items-center space-x-4 mb-6">
            <div className="bg-blue-600 p-2 rounded-full">
              <User className="w-6 h-6 text-white" />
            </div>
            <div>
              <h2 className="font-semibold">{account?.name}</h2>
              <p className="text-sm text-gray-400">{account?.email || `ID: ${accountId}`}</p>
            </div>
          </div>

          <div className="text-sm text-gray-300 space-y-2 mb-6">
            <p>Total Batches: <span className="font-semibold text-white">{batches.length}</span></p>
            <p>Active: <span className="text-orange-400">{batches.filter(b => !b.completed).length}</span></p>
            <p>Completed: <span className="text-green-400">{batches.filter(b => b.completed).length}</span></p>
          </div>

          <div>
            <h3 className="text-sm text-gray-300 mb-2">Select Batch</h3>
            <ul className="space-y-2">
              {batches.map((batch) => (
                <li key={batch.id}>
                  <button
                    className={`w-full text-left p-2 rounded-md border ${
                      selectedBatch?.id === batch.id
                        ? 'bg-blue-900/50 border-blue-400 text-blue-200'
                        : 'border-gray-600 text-gray-300 hover:bg-gray-700/50'
                    }`}
                    onClick={() => setSelectedBatch(batch)}
                  >
                    <div className="flex justify-between items-center">
                      <span>{batch.meta?.name || `Batch ${batch.id}`}</span>
                      {batch.completed ? (
                        <Trophy className="w-4 h-4 text-green-400" />
                      ) : (
                        <Clock className="w-4 h-4 text-orange-400" />
                      )}
                    </div>
                    <p className="text-xs text-gray-400">
                      {batch.bets?.length || 0} bets • ${calculateTotalStake(batch.bets || [])}
                    </p>
                  </button>
                </li>
              ))}
            </ul>
          </div>
        </aside>

        {/* Batch Section */}
        <section className="lg:col-span-3 space-y-6">
          {selectedBatch && (
            <>
              {/* Metadata */}
              <div className="bg-gray-800 rounded-xl border border-gray-700 p-4">
                <div className="flex justify-between items-center mb-4">
                  <h1 className="text-xl font-bold">{selectedBatch.meta?.name || `Batch ${selectedBatch.id}`}</h1>
                  <span className={`px-3 py-1 text-sm rounded-full border ${
                    selectedBatch.completed
                      ? 'text-green-300 border-green-600 bg-green-900/20'
                      : 'text-orange-300 border-orange-600 bg-orange-900/20'
                  }`}>
                    {selectedBatch.completed ? 'Completed' : 'Active'}
                  </span>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 text-sm text-gray-300">
                  <div className="flex items-center space-x-2">
                    <Calendar className="w-4 h-4" />
                    <span>{formatDate(selectedBatch.created_at)}</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Target className="w-4 h-4" />
                    <span>Total Bets: {selectedBatch.bets?.length || 0}</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <DollarSign className="w-4 h-4" />
                    <span>Total Stake: ${calculateTotalStake(selectedBatch.bets || [])}</span>
                  </div>
                </div>
              </div>

              {/* Bets Table */}
              <div className="bg-gray-800 rounded-xl border border-gray-700 overflow-x-auto">
                <table className="min-w-full text-sm text-left">
                  <thead className="bg-gray-700 text-gray-300 uppercase">
                    <tr>
                      <th className="px-4 py-2">#</th>
                      <th className="px-4 py-2">Selection</th>
                      <th className="px-4 py-2">Stake</th>
                      <th className="px-4 py-2">Cost</th>
                    </tr>
                  </thead>
                  <tbody>
                    {selectedBatch.bets?.map((bet, index) => (
                      <tr key={bet.id} className="border-t border-gray-700 hover:bg-gray-700/40">
                        <td className="px-4 py-2">{index + 1}</td>
                        <td className="px-4 py-2">{bet.selection}</td>
                        <td className="px-4 py-2">${bet.stake}</td>
                        <td className="px-4 py-2">${bet.cost}</td>
                      </tr>
                    ))}
                    <tr className="bg-gray-700 border-t border-gray-600 font-semibold">
                      <td colSpan="2" className="px-4 py-2 text-right">Total</td>
                      <td className="px-4 py-2">${calculateTotalStake(selectedBatch.bets)}</td>
                      <td className="px-4 py-2"></td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </>
          )}
        </section>
      </div>
    </div>
  );
}
