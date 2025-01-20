import React from 'react';
import Inflation from './components/Inflation';
import TBill from './components/TBill';
import RealYield from './components/RealYield';
import LongTermRates from './components/LongTermRates';

function App() {
  return (
    <div className="App p-4">
      <h1 className="text-2xl font-bold mb-6">Macroeconomic Dashboard</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Inflation />
        <TBill />
        <RealYield />
      </div>
      <div className="mt-6">
        <LongTermRates />
      </div>
    </div>
  );
}

export default App;