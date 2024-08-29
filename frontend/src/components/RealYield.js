import React, { useEffect, useState } from 'react';
import axios from 'axios';

function RealYield() {
  const [realYield, setRealYield] = useState(null);

  useEffect(() => {
    axios.get('http://localhost:3030/api/v1/real_yield')
      .then(response => setRealYield(response.data))
      .catch(error => console.error('Error fetching real yield data:', error));
  }, []);

  return (
    <div>
      <h2>Real T-Bill Yield</h2>
      {realYield !== null ? <p>{realYield}%</p> : <p>Loading...</p>}
    </div>
  );
}

export default RealYield;
