import React from 'react';
import Inflation from './components/Inflation';
import TBill from './components/TBill';
import RealYield from './components/RealYield';

function App() {
  return (
    <div className="App">
      <h1>Macroeconomic Dashboard</h1>
      <Inflation />
      <TBill />
      <RealYield />
    </div>
  );
}

export default App;
