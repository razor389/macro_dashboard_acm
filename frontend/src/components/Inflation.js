import React, { useEffect, useState } from 'react';
import axios from 'axios';

function Inflation() {
  const [inflation, setInflation] = useState(null);

  useEffect(() => {
    axios.get('http://localhost:3030/api/v1/inflation')
      .then(response => setInflation(response.data))
      .catch(error => console.error('Error fetching inflation data:', error));
  }, []);

  return (
    <div>
      <h2>Current Inflation Rate</h2>
      {inflation !== null ? <p>{inflation}%</p> : <p>Loading...</p>}
    </div>
  );
}

export default Inflation;
