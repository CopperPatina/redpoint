import type { WorkoutSession, ExerciseEntry} from "./types";
import { writeFileSync, readFileSync } from "fs";

export function totalVolume(session: WorkoutSession): number {
    let sum = 0;
    for (const exercise of session.exercises) {
        sum += exercise.reps * exercise.sets * exercise.weightLb;
    }
    return sum;
  }

  export function volumeByLift(session: WorkoutSession): Record<string, number>{
    let liftMap: Record<string, number> = {};
    for (const lift of session.exercises) {
        liftMap[lift.name] = lift.sets * lift.reps * lift.weightLb;
    }
    return liftMap;
  }

  export function mainLiftSummary(session: WorkoutSession): void {
    let mainLift: ExerciseEntry[] = session.exercises.filter(e => e.isMainLift);
    if (mainLift.length == 0) {
        console.log(`No lift marked as main lift`);
        return;
    }

    for (const lift of mainLift) {
        console.log(`MAIN LIFT: ${lift.name} sets: ${lift.sets} total reps: ${lift.sets * lift.reps} weight ${lift.weightLb}`)
    }
  }

  export function saveWorkoutLog(workout: WorkoutSession, file: string): void {
    const json = JSON.stringify(workout, null, 2);
    try {
        writeFileSync(file, json, "utf-8");
    } catch(error) {
        console.error(`Error saving file ${error}`);
    }
  }

  export function loadWorkoutLog(file: string): WorkoutSession | null {
    try {
        let workout = readFileSync(file, "utf-8");
        let session: WorkoutSession = JSON.parse(workout);
        return session;
    } catch(error) {
        console.error(`Error loading file ${error}`);
        return null;
    }
  }