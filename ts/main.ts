import * as climblog from "./index";


function workoutFileName(workout: climblog.WorkoutSession): string {
    return "workout-" + workout.date;
  }

function climbFileName(climb: climblog.ClimbingSession): string {
    return "climb-" + climb.date
}

function climbMetricsFileName(climb: climblog.ClimbMetricsEntry): string {
    return "metrics-" + climb.date
}

function main() {
    //let filePath = "../logs/" + workoutFileName(w);
    //climblog.saveWorkoutLog(w, filePath);
    let wo = climblog.loadWorkoutLog(filePath);
    if (wo) {
        console.log(`Workout on ${wo.date}`);
        for (const e of wo.exercises) {
          console.log(`- ${e.name}: ${e.sets}x${e.reps} @ ${e.weightLb}lb`);
        }
      }
  }

  main();