examples:

const session: ClimbingSession = {
    date: "04-06-2025",
    location: "Movement",
    style: "rope",
    climbs: [
        {grade: "5.9", attempts: 1, sent: true, reached_top: true, lead: true, rests: 0},
        {grade: "5.11a", attempts: 1, sent: true, reached_top: true, lead: true, rests: 3},
        {grade: "5.12a", attempts: 1, sent: false, reached_top: false, lead: true}
    ]
  };

const workout: WorkoutSession = {
    date: "04-06-2025",
    exercises: [
        {name: "deadlift", sets: 4, reps: 3, weightLb: 220, isMainLift: true},
        {name: "squat", sets: 3, reps: 10, weightLb: 150, isMainLift: false},
        {name: "bulgarian split squat", sets: 3, reps: 8, weightLb: 50, isMainLift: false}
    ]
}