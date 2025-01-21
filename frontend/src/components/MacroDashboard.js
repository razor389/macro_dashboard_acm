import React, { useState, useEffect } from 'react';
import axios from 'axios';

const MacroDashboard = () => {
  const [data, setData] = useState({
    inflation: null,
    tbill: null,
    longTermRates: null
  });
  const [estimatedInflation, setEstimatedInflation] = useState(2.5);
  const [estimatedGrowth, setEstimatedGrowth] = useState(1.0);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [inflationRes, tbillRes, longTermRes] = await Promise.all([
          axios.get('http://localhost:3030/api/v1/inflation'),
          axios.get('http://localhost:3030/api/v1/tbill'),
          axios.get('http://localhost:3030/api/v1/long_term_rates')
        ]);

        setData({
          inflation: inflationRes.data,
          tbill: tbillRes.data,
          longTermRates: longTermRes.data
        });
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    fetchData();
  }, []);

  const effectiveRealYield = data.tbill !== null ? (data.tbill - estimatedInflation).toFixed(2) : null;
  const marketInflation = data.longTermRates ? (data.longTermRates.bond_yield - data.longTermRates.tips_yield).toFixed(2) : null;
  const deltaInflation = data.longTermRates ? (marketInflation - estimatedInflation).toFixed(2) : null;
  const deltaGrowth = data.longTermRates ? (data.longTermRates.tips_yield - estimatedGrowth).toFixed(2) : null;
  const estimatedReturns = data.longTermRates ? 
    (data.longTermRates.bond_yield + parseFloat(deltaInflation) + parseFloat(deltaGrowth)).toFixed(2) : null;

  return (
    <div className="p-6 max-w-4xl mx-auto bg-white">
      <h1 className="text-3xl font-bold mb-6">Macroeconomic Dashboard</h1>
      
      <div className="grid gap-6">
        {/* Current Market Data Section */}
        <div className="space-y-4">
          <div className="bg-gray-50 p-4 rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Current Market Data</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <p className="text-gray-600">Current Inflation Rate</p>
                <p className="text-2xl font-bold">{data.inflation !== null ? `${data.inflation.toFixed(2)}%` : 'Loading...'}</p>
              </div>
              <div>
                <p className="text-gray-600">Current (Nominal) 4wk T-Bill Yield</p>
                <p className="text-2xl font-bold">{data.tbill !== null ? `${data.tbill.toFixed(2)}%` : 'Loading...'}</p>
              </div>
            </div>
          </div>

          {/* Parameters Section */}
          <div className="bg-gray-50 p-4 rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Parameters</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-gray-600">
                  Estimated Inflation
                  <input
                    type="number"
                    step="0.1"
                    value={estimatedInflation}
                    onChange={(e) => setEstimatedInflation(parseFloat(e.target.value))}
                    className="ml-4 p-1 border rounded"
                  />
                  %
                </label>
              </div>
              <div>
                <p className="text-gray-600">Effective Real T-Bill Yield</p>
                <p className="text-2xl font-bold">{effectiveRealYield !== null ? `${effectiveRealYield}%` : 'Loading...'}</p>
              </div>
            </div>
          </div>

          {/* Long-term Analysis Section */}
          <div className="bg-gray-50 p-4 rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Long-term Analysis</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <p className="text-gray-600">20-year Bond Yield</p>
                <p className="text-xl font-bold">
                  {data.longTermRates?.bond_yield ? `${data.longTermRates.bond_yield.toFixed(2)}%` : 'Loading...'}
                </p>
              </div>
              <div>
                <p className="text-gray-600">Horizon Premium (20yr TIPS yield)</p>
                <p className="text-xl font-bold">
                  {data.longTermRates?.tips_yield ? `${data.longTermRates.tips_yield.toFixed(2)}%` : 'Loading...'}
                </p>
              </div>
              <div>
                <label className="block text-gray-600">
                  20yr Real GDP Growth Estimate
                  <input
                    type="number"
                    step="0.1"
                    value={estimatedGrowth}
                    onChange={(e) => setEstimatedGrowth(parseFloat(e.target.value))}
                    className="ml-4 p-1 border rounded"
                  />
                  %
                </label>
              </div>
            </div>
          </div>

          {/* Final Analysis Section */}
          <div className="bg-gray-50 p-4 rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Analysis</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <p className="text-gray-600">Δ Inflation</p>
                <p className="text-xl font-bold">{deltaInflation !== null ? `${deltaInflation}%` : 'Loading...'}</p>
              </div>
              <div>
                <p className="text-gray-600">Δ Growth</p>
                <p className="text-xl font-bold">{deltaGrowth !== null ? `${deltaGrowth}%` : 'Loading...'}</p>
              </div>
              <div>
                <p className="text-gray-600">Estimated Returns</p>
                <p className="text-xl font-bold">{estimatedReturns !== null ? `${estimatedReturns}%` : 'Loading...'}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MacroDashboard;