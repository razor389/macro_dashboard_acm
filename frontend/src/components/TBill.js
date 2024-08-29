import React, { useEffect, useState } from 'react';
import axios from 'axios';

function TBill() {
  const [tbill, setTbill] = useState(null);

  useEffect(() => {
    axios.get('http://localhost:3030/api/v1/tbill')
      .then(response => setTbill(response.data))
      .catch(error => console.error('Error fetching T-bill data:', error));
  }, []);

  return (
    <div>
      <h2>Current T-Bill Rate</h2>
      {tbill !== null ? <p>{tbill}%</p> : <p>Loading...</p>}
    </div>
  );
}

export default TBill;
