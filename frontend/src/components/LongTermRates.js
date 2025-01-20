import React, { useState, useEffect } from 'react';
import axios from 'axios';

function LongTermRates() {
  const [rates, setRates] = useState(null);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);
  const [estimatedInflation, setEstimatedInflation] = useState(2.5);
  const [estimatedHorizonPremium, setEstimatedHorizonPremium] = useState(1.0);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        setError(null);
        console.log('Fetching long-term rates...');
        const response = await axios.get('http://localhost:3030/api/v1/long_term_rates');
        console.log('Long-term rates response:', response.data);
        setRates(response.data);
      } catch (err) {
        console.error('Error fetching long-term rates:', err);
        setError(err.message || 'Failed to fetch long-term rates');
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) return <div>Loading long-term rates...</div>;
  if (error) return <div>Error: {error}</div>;
  if (!rates) return <div>No data available</div>;

  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">Long-term Rates Analysis</h2>
      
      <div className="mb-4">
        <h3 className="font-semibold">Market Data</h3>
        <div className="grid grid-cols-2 gap-2">
          <div>20-year Bond Yield:</div>
          <div>{rates.bond_yield?.toFixed(2)}%</div>
          <div>20-year TIPS Yield:</div>
          <div>{rates.tips_yield?.toFixed(2)}%</div>
          <div>Market-implied Inflation:</div>
          <div>{rates.market_inflation?.toFixed(2)}%</div>
          <div>Real T-Bill Rate:</div>
          <div>{rates.real_tbill?.toFixed(2)}%</div>
        </div>
      </div>

      <div className="mb-4">
        <h3 className="font-semibold">Parameters</h3>
        <div className="flex gap-4">
          <label className="flex items-center">
            Estimated Inflation:
            <input
              type="number"
              step="0.1"
              value={estimatedInflation}
              onChange={e => setEstimatedInflation(parseFloat(e.target.value))}
              className="ml-2 p-1 border rounded"
            />%
          </label>
          <label className="flex items-center">
            Estimated Horizon Premium:
            <input
              type="number"
              step="0.1"
              value={estimatedHorizonPremium}
              onChange={e => setEstimatedHorizonPremium(parseFloat(e.target.value))}
              className="ml-2 p-1 border rounded"
            />%
          </label>
        </div>
      </div>

      <div>
        <h3 className="font-semibold">Analysis</h3>
        <div className="grid grid-cols-2 gap-2">
          <div>Current Horizon Premium:</div>
          <div>{rates.horizon_premium?.toFixed(2)}%</div>
          <div>Δ Inflation Expectations:</div>
          <div>{rates.delta_inflation?.toFixed(2)}%</div>
          <div>Δ Horizon Premiums:</div>
          <div>{rates.delta_horizon?.toFixed(2)}%</div>
          <div>Estimated Bond Returns:</div>
          <div>{rates.estimated_returns?.toFixed(2)}%</div>
        </div>
      </div>
    </div>
  );
}

export default LongTermRates;