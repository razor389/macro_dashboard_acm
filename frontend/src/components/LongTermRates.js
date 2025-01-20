// LongTermRates.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';

function LongTermRates() {
  const [rawRates, setRawRates] = useState(null);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);

  // Local parameters that the user can edit
  const [estimatedInflation, setEstimatedInflation] = useState(2.5);
  const [estimatedHorizonPremium, setEstimatedHorizonPremium] = useState(1.0);

  // Fetch the raw yields from the backend
  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        setError(null);
        const response = await axios.get('http://localhost:3030/api/v1/long_term_rates');
        setRawRates(response.data); // { bond_yield, tips_yield, real_tbill }
      } catch (err) {
        setError(err.message || 'Failed to fetch');
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  if (loading) return <div>Loading...</div>;
  if (error)   return <div>Error: {error}</div>;
  if (!rawRates) return <div>No data</div>;

  // Derive everything else on the client side:
  const marketInflation = rawRates.bond_yield - rawRates.tips_yield;
  const horizonPremium = rawRates.tips_yield - rawRates.real_tbill;

  const deltaInflation = marketInflation - estimatedInflation;
  const deltaHorizon = horizonPremium - estimatedHorizonPremium;
  const estimatedReturns = rawRates.bond_yield + deltaInflation + deltaHorizon;

  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">Long-term Rates Analysis</h2>

      <div>
        <h3>Market Data</h3>
        <p>20-year Bond Yield: {rawRates.bond_yield?.toFixed(2)}%</p>
        <p>20-year TIPS Yield: {rawRates.tips_yield?.toFixed(2)}%</p>
        <p>Real T-Bill Rate: {rawRates.real_tbill?.toFixed(2)}%</p>
        <p>Market-implied Inflation: {marketInflation.toFixed(2)}%</p>
        <p>Horizon Premium: {horizonPremium.toFixed(2)}%</p>
      </div>

      <div>
        <h3>Parameters</h3>
        <label>
          Estimated Inflation:
          <input
            type="number"
            step="0.1"
            value={estimatedInflation}
            onChange={e => setEstimatedInflation(parseFloat(e.target.value))}
          />
          %
        </label>
        <br />
        <label>
          Estimated Horizon Premium:
          <input
            type="number"
            step="0.1"
            value={estimatedHorizonPremium}
            onChange={e => setEstimatedHorizonPremium(parseFloat(e.target.value))}
          />
          %
        </label>
      </div>

      <div>
        <h3>Analysis</h3>
        <p>Δ Inflation: {deltaInflation.toFixed(2)}%</p>
        <p>Δ Horizon: {deltaHorizon.toFixed(2)}%</p>
        <p>Estimated Returns: {estimatedReturns.toFixed(2)}%</p>
      </div>
    </div>
  );
}

export default LongTermRates;
