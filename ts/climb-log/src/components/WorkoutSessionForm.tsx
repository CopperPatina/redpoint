import React, { useState, useEffect } from 'react';
import { WorkoutSession, ExerciseEntry } from '../types/types';
import { safeJSONParse } from '../utils/storage';

const defaultExercises = [
  "deadlift", "squat", "pullups", "dips", "bench press",
  "overhead press", "bulgarian split squat", "face pulls",
  "tricep pulldown", "lat pulldown", "pistol squats", "L sits",
  "narrow grip bench"
];

const getDefaultSession = (): WorkoutSession => ({
  date: new Date().toISOString().split("T")[0],
  notes: '',
  exercises: [],
});

const WorkoutSessionForm: React.FC = () => {
  const [session, setSession] = useState<WorkoutSession>(getDefaultSession());
  const [savedExercises, setSavedExercises] = useState<string[]>(() => {
    return safeJSONParse("savedExercises", defaultExercises);
  });
  const [savedTemplates, setSavedTemplates] = useState<{ [key: string]: ExerciseEntry[] }>(() => {
    return safeJSONParse("workoutTemplates", {});
  });
  const [selectedTemplate, setSelectedTemplate] = useState<string>("");
  const [newExercise, setNewExercise] = useState("");
  const [showExerciseEditor, setShowExerciseEditor] = useState(false);
  const [submitted, setSubmitted] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (submitted) {
      const timer = setTimeout(() => setSubmitted(false), 4000);
      return () => clearTimeout(timer);
    }
  }, [submitted]);

  const saveExerciseList = (list: string[]) => {
    localStorage.setItem("savedExercises", JSON.stringify(list));
    setSavedExercises(list);
  };


  const saveTemplateList = (list: { [key: string]: ExerciseEntry[] }) => {
    localStorage.setItem("workoutTemplates", JSON.stringify(list));
    setSavedTemplates(list);
  };

  const loadWorkoutTemplate = (templateName: string) => {
    const template = savedTemplates[templateName];
    if (template) {
      setSession(prev => ({
        ...prev,
        exercises: template,
      }));
    }
  };

  const addExerciseToSession = () => {
    setSession(prev => ({
      ...prev,
      exercises: [...prev.exercises, {
        name: '',
        sets: 0,
        reps: 0,
        weightLb: 0,
        rpe: 0,
        isMainLift: false,
      }]
    }));
  };

  const addCustomExercise = () => {
    if (newExercise && !savedExercises.includes(newExercise)) {
      const updated = [...savedExercises, newExercise];
      saveExerciseList(updated);
      setNewExercise("");
    }
  };

  const removeSavedExercise = (name: string) => {
    const updated = savedExercises.filter(e => e !== name);
    saveExerciseList(updated);
  };

  async function submitSession() {
    setError(null);
    if (!session.date || session.exercises.length === 0) {
      setError("Please fill out the date and add at least one exercise.");
      return;
    }
    for (const exercise of session.exercises) {
      if (!exercise.sets || !exercise.reps || !exercise.weightLb) {
        setError("Each exercise must have set, reps and a weight.");
        return;
      }
    }

    try {
      const res = await fetch("http://localhost:3000/api/logs/workout", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(session),
      });

      if (res.ok) {
        setSubmitted(true);
        setSession(getDefaultSession());
        console.log("Workout session logged");
      } else {
        setError("Failed to log workout session.");
        console.error("Failed to log workout session");
      }
    } catch (err) {
      setError("Network or server error. Please try again.");
      console.error("Error with network or server");
    }
  }

  const updateWorkout = (index: number, updated: ExerciseEntry) => {
    const exercises = [...session.exercises];
    exercises[index] = updated;
    setSession({ ...session, exercises });
  };

  const removeExerciseFromSession = (index: number) => {
    const exercises = session.exercises.filter((_, i) => i !== index);
    setSession({ ...session, exercises });
  };

  return (
    <div className="max-w-3xl mx-auto p-6 sm:p-10 bg-white min-h-screen text-gray-800">
      {submitted && (
        <div className="mb-6 p-4 bg-green-100 border border-green-300 text-green-800 rounded">
          Workout session saved successfully!
        </div>
      )}
      {error && (
        <div className="mb-6 p-4 bg-red-100 border border-red-300 text-red-800 rounded">
          {error}
        </div>
      )}
      <form className="space-y-10">
        <div className="space-y-6 border-b pb-6">
          <h2 className="text-xl font-bold text-gray-700">Session Info</h2>

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
            <label className="font-medium text-sm">Notes</label>
            <textarea
              value={session.notes}
              onChange={(e) => setSession({ ...session, notes: e.target.value })}
              className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
            />
          </div>

          <div>
            <button
              type="button"
              onClick={() => setShowExerciseEditor(prev => !prev)}
              className="text-sm text-blue-600 hover:underline"
            >
              {showExerciseEditor ? "Hide Exercise List" : "Edit Saved Exercises"}
            </button>

            {showExerciseEditor && (
              <div className="mt-4 space-y-4">
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={newExercise}
                    onChange={(e) => setNewExercise(e.target.value)}
                    placeholder="New exercise"
                    className="border border-gray-300 rounded px-4 py-2 w-full focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                  <button
                    type="button"
                    onClick={addCustomExercise}
                    className="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
                  >
                    Add
                  </button>
                </div>
                <div className="flex flex-wrap gap-2">
                  {savedExercises.map((ex) => (
                    <span
                      key={ex}
                      className="bg-gray-200 rounded-full px-3 py-1 inline-flex items-center"
                    >
                      {ex}
                      <button
                        onClick={() => removeSavedExercise(ex)}
                        className="ml-2 text-red-500 hover:text-red-700 text-xs"
                      >
                        ×
                      </button>
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>

          <div className="flex gap-4 pt-4">
            <button
              type="button"
              onClick={addExerciseToSession}
              className="px-5 py-2 bg-emerald-600 text-white rounded-md hover:bg-emerald-700 shadow"
            >
              Add Exercise
            </button>
            <button
              type="button"
              onClick={submitSession}
              className="px-5 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 shadow"
            >
              Log Session
            </button>
          </div>

          <div className="flex gap-4 pt-4">
            <button
              type="button"
              onClick={() => {
                const name = prompt("Enter a name for this workout template:");
                if (name) {
                  const updatedTemplates = { ...savedTemplates, [name]: session.exercises };
                  saveTemplateList(updatedTemplates);
                }
              }}
              className="px-5 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 shadow"
            >
              Save As Template
            </button>
          </div>

          <div className="flex flex-col gap-1">
          <label className="font-medium text-sm">Template</label>
          <select
            value={selectedTemplate}
            onChange={(e) => {
              const selected = e.target.value;
              setSelectedTemplate(selected);
              if (selected) {
                loadWorkoutTemplate(selected);
              }
            }}
            className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Load workout template</option>
            {Object.keys(savedTemplates).map((templateName) => (
              <option key={templateName} value={templateName}>
                {templateName}
              </option>
            ))}
          </select>
        </div>
      </div>

        <div className="space-y-10">
          <h2 className="text-xl font-bold text-gray-700">Exercises</h2>

          {session.exercises.map((exercise, index) => (
            <div key={index} className="p-6 border rounded-xl bg-gray-50 shadow space-y-6">
              <div className="flex flex-col gap-1">
                <label className="font-medium text-sm">Exercise</label>
                <select
                  value={exercise.name}
                  onChange={(e) => updateWorkout(index, { ...exercise, name: e.target.value })}
                  className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="">Select exercise</option>
                  {savedExercises.map((g) => (
                    <option key={g} value={g}>{g}</option>
                  ))}
                </select>
              </div>

              <div className="grid grid-cols-2 sm:grid-cols-3 gap-4">
                <div className="flex flex-col gap-1">
                  <label className="text-sm">Sets</label>
                  <input
                    type="number"
                    value={exercise.sets || 0}
                    onChange={(e) => updateWorkout(index, { ...exercise, sets: Number(e.target.value) })}
                    className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div className="flex flex-col gap-1">
                  <label className="text-sm">Reps</label>
                  <input
                    type="number"
                    value={exercise.reps || 0}
                    onChange={(e) => updateWorkout(index, { ...exercise, reps: Number(e.target.value) })}
                    className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div className="flex flex-col gap-1">
                  <label className="text-sm">Weight (lb)</label>
                  <input
                    type="number"
                    value={exercise.weightLb || 0}
                    onChange={(e) => updateWorkout(index, { ...exercise, weightLb: Number(e.target.value) })}
                    className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div className="flex flex-col gap-1">
                  <label className="text-sm">RPE (optional)</label>
                  <input
                    type="number"
                    value={exercise.rpe || 0}
                    onChange={(e) => updateWorkout(index, { ...exercise, rpe: Number(e.target.value) })}
                    className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
              </div>

              <div className="pt-2">
                <label className="flex items-center gap-2 text-sm">
                  <input
                    type="checkbox"
                    checked={exercise.isMainLift || false}
                    onChange={(e) => updateWorkout(index, { ...exercise, isMainLift: e.target.checked })}
                  />
                  Main Lift
                </label>
              </div>

              <div className="text-right">
                <button
                  type="button"
                  onClick={() => removeExerciseFromSession(index)}
                  className="text-red-600 hover:text-red-800 text-sm font-medium"
                >
                  Remove Exercise
                </button>
              </div>
            </div>
          ))}
        </div>
      </form>
    </div>
  );
};

export default WorkoutSessionForm;