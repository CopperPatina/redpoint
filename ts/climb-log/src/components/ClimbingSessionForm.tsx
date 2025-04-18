import React, { useState, useEffect } from 'react';
import { ClimbingSession, ClimbEntry } from '../types/types';

const ropeGrades = [
  "5.6", "5.7", "5.8", "5.9",
  "5.10a", "5.10b", "5.10c", "5.10d",
  "5.11a", "5.11b", "5.11c", "5.11d",
  "5.12a", "5.12b", "5.12c", "5.12d",
  "5.13a", "5.13b", "5.13c", "5.13d",
  "5.14a",
];

const boulderGrades = [
  "V0", "V1", "V2", "V3", "V4", "V5",
  "V6", "V7", "V8", "V9", "V10",
];

const getDefaultSession = (): ClimbingSession => ({
  date: new Date().toISOString().split("T")[0],
  location: 'Movement Columbia',
  style: 'boulder',
  notes: '',
  climbs: [],
});

const ClimbingSessionForm: React.FC = () => {
  const [session, setSession] = useState<ClimbingSession>(getDefaultSession());
  const [submitted, setSubmitted] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (submitted) {
      const timer = setTimeout(() => setSubmitted(false), 4000);
      return () => clearTimeout(timer);
    }
  }, [submitted]);

  const addClimb = () => {
    setSession(prev => ({
      ...prev,
      climbs: [...prev.climbs, {
        name: '',
        grade: '',
        attempts: 1,
        sent: false,
        reachedTop: false,
        lead: false,
        rests: 0,
      }]
    }));
  };

  async function submitSession() {
    setError(null);
    if (!session.date || !session.location || session.climbs.length === 0) {
      setError("Please fill out the date, location, and add at least one climb.");
      return;
    }
    for (const climb of session.climbs) {
      if (!climb.grade) {
        setError("Each climb must have a grade.");
        return;
      }
    }

    try {
      const res = await fetch("http://localhost:3000/api/logs/climb", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(session),
      });

      if (res.ok) {
        setSubmitted(true);
        setSession(getDefaultSession());
        console.log("Climbing session logged");
      } else {
        setError("Failed to log climbing session.");
        console.error("Failed to log climbing session");
      }
    } catch (err) {
      setError("Network or server error. Please try again.");
      console.error("Error with network or server");
    }
  }

  const updateClimb = (index: number, updated: ClimbEntry) => {
    const climbs = [...session.climbs];
    climbs[index] = updated;
    setSession({ ...session, climbs });
  };

  const removeClimb = (index: number) => {
    const climbs = session.climbs.filter((_, i) => i !== index);
    setSession({ ...session, climbs });
  };

  const gradeOptions = session.style === "rope" ? ropeGrades : boulderGrades;

  return (
    <div className="max-w-3xl mx-auto p-10 bg-white min-h-screen text-gray-800">
      {submitted && (
        <div className="mb-6 p-4 bg-green-100 border border-green-300 text-green-800 rounded">
          Climbing session saved successfully!
        </div>
      )}
      {error && (
        <div className="mb-6 p-4 bg-red-100 border border-red-300 text-red-800 rounded">
          {error}
        </div>
      )}
      <form className="space-y-10">
        <div className="space-y-6 border-b pb-6">
          <h2 className="text-xl font-bold text-gray-700">🧗 Session Info</h2>

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
            <label className="font-medium text-sm">Location</label>
            <input
              type="text"
              value={session.location}
              onChange={(e) => setSession({ ...session, location: e.target.value })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex flex-col gap-1">
            <label className="font-medium text-sm">Style</label>
            <select
              value={session.style}
              onChange={(e) => setSession({ ...session, style: e.target.value as 'boulder' | 'rope' })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="boulder">Boulder</option>
              <option value="rope">Rope</option>
            </select>
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

          <div className="flex gap-4 pt-2">
            <button
              type="button"
              onClick={addClimb}
              className="px-5 py-2 bg-emerald-600 text-white rounded-md hover:bg-emerald-700 shadow"
            >
              Add Climb
            </button>
            <button
              type="button"
              onClick={submitSession}
              className="px-5 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 shadow"
            >
              Log Session
            </button>
          </div>
        </div>

        <div className="space-y-10">
          <h2 className="text-xl font-bold text-gray-700">🧗 Climb Attempts</h2>

          {session.climbs.map((climb, index) => (
            <div key={index} className="p-6 border rounded-xl bg-gray-50 shadow space-y-6">
              <div className="flex flex-col gap-1">
                <label className="font-medium text-sm">Name</label>
                <input
                  type="text"
                  value={climb.name || ''}
                  onChange={(e) => updateClimb(index, { ...climb, name: e.target.value })}
                  className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div className="flex flex-col gap-1">
                <label className="font-medium text-sm">Grade</label>
                <select
                  value={climb.grade}
                  onChange={(e) => updateClimb(index, { ...climb, grade: e.target.value })}
                  className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="">Select grade</option>
                  {gradeOptions.map((g) => (
                    <option key={g} value={g}>{g}</option>
                  ))}
                </select>
              </div>

              {session.style === "rope" && (
                <div className="flex flex-col gap-1">
                  <label className="font-medium text-sm">Rests</label>
                  <input
                    type="number"
                    value={climb.rests || 0}
                    onChange={(e) => updateClimb(index, { ...climb, rests: Number(e.target.value) })}
                    className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
              )}

              <div className="grid grid-cols-2 sm:grid-cols-3 gap-4">
                <label className="flex items-center gap-2 text-sm">
                  <input
                    type="checkbox"
                    checked={climb.sent}
                    onChange={(e) => updateClimb(index, { ...climb, sent: e.target.checked })}
                  />
                  Sent
                </label>
                <label className="flex items-center gap-2 text-sm">
                  <input
                    type="checkbox"
                    checked={climb.reachedTop}
                    onChange={(e) => updateClimb(index, { ...climb, reachedTop: e.target.checked })}
                  />
                  Reached Top
                </label>
              </div>

              {session.style === "rope" && (
                <div className="pt-2">
                  <label className="flex items-center gap-2 text-sm">
                    <input
                      type="checkbox"
                      checked={climb.lead}
                      onChange={(e) => updateClimb(index, { ...climb, lead: e.target.checked })}
                    />
                    Lead
                  </label>
                </div>
              )}

              <div className="text-right">
                <button
                  type="button"
                  onClick={() => removeClimb(index)}
                  className="text-red-600 hover:text-red-800 text-sm font-medium"
                >
                  Remove Climb
                </button>
              </div>
            </div>
          ))}
        </div>
      </form>
    </div>
  );
};

export default ClimbingSessionForm;