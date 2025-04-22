import React, { useState, useEffect } from 'react';
import { ClimbMetricsEntry } from '../types/types';

const getDefaultSession = (): ClimbMetricsEntry => ({
  date: new Date().toISOString().split("T")[0],
  fingerStrengthPercentBW: 0,
  maxPullupPercentBW: 0,
  notes: ''
});

const ClimbingMetricsForm: React.FC = () => {
  const [session, setSession] = useState<ClimbMetricsEntry>(getDefaultSession());
  const [submitted, setSubmitted] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (submitted) {
      const timer = setTimeout(() => setSubmitted(false), 4000);
      return () => clearTimeout(timer);
    }
  }, [submitted]);

  const addMetrics = () => {
    setSession(prev => ({
      ...prev
    }));
  };

  async function submitSession() {
    setError(null);
    if (!session.date) {
      setError("Please fill out the date.");
      return;
    }


    try {
      const res = await fetch("http://localhost:3000/api/logs/metrics", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(session),
      });

      if (res.ok) {
        setSubmitted(true);
        setSession(getDefaultSession());
        console.log("Metrics session logged");
      } else {
        setError("Failed to log metrics session.");
        console.error("Failed to log metrics session");
      }
    } catch (err) {
      setError("Network or server error. Please try again.");
      console.error("Error with network or server");
    }
  }


  return (
    <div className="max-w-3xl mx-auto p-10 bg-white min-h-screen text-gray-800">
      {submitted && (
        <div className="mb-6 p-4 bg-green-100 border border-green-300 text-green-800 rounded">
          Metrics session saved successfully!
        </div>
      )}
      {error && (
        <div className="mb-6 p-4 bg-red-100 border border-red-300 text-red-800 rounded">
          {error}
        </div>
      )}
      <form className="space-y-10">
        <div className="space-y-6 border-b pb-6">
          <h2 className="text-xl font-bold text-gray-700">Metrics Info</h2>

          <div className="flex flex-col gap-1">
            <label className="font-medium text-sm">Date</label>
            <input
              type="date"
              value={session.date}
              onChange={(e) => setSession({ ...session, date: e.target.value })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex flex-col gap-1">
            <label className="font-medium text-sm">Finger Strength (% Bodyweight)</label>
            <input
              type="number"
              value={session.fingerStrengthPercentBW}
              onChange={(e) => setSession({ ...session, fingerStrengthPercentBW: Number(e.target.value) })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex flex-col gap-1">
            <label className="font-medium text-sm">Max Pullup (% Bodyweight)</label>
            <input
              type="number"
              value={session.maxPullupPercentBW}
              onChange={(e) => setSession({ ...session, maxPullupPercentBW: Number(e.target.value)  })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex flex-col gap-1">
            <label className="font-medium text-sm">Notes</label>
            <textarea
              value={session.notes}
              onChange={(e) => setSession({ ...session, notes: e.target.value })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
            />
          </div>

        </div>
      </form>
    </div>
  );
};

export default ClimbingMetricsForm;