import type { ClimbEntry, ClimbStyle, ClimbingSession, ClimbMetricsEntry } from "./types";
import type { WorkoutSession, ExerciseEntry} from "./types";

  function printSentClimbs(session) {
    let sentClimbs: ClimbEntry[] = session.climbs.filter(c => c.sent);
    for (const climb of sentClimbs) {
        console.log(`grade: ${climb.grade}, attempts: ${climb.attempts}, rests ${climb.rests ?? "unknown"}`);
    }
  }

  function averageRests(session: ClimbingSession): number {
    const restClimbs: ClimbEntry[] = session.climbs.filter(c => c.rests !== undefined);
  
    const totalRests = restClimbs.reduce((sum, climb) => sum + (climb.rests ?? 0), 0);
  
    return restClimbs.length === 0 ? 0 : totalRests / restClimbs.length;
  }

  function totalVolume(session: WorkoutSession): number {
    let sum = 0;
    for (const exercise of session.exercises) {
        sum += exercise.reps * exercise.sets * exercise.weightLb;
    }
    return sum;
  }

  function volumeByLift(session: WorkoutSession): Record<string, number>{
    let liftMap: Record<string, number> = {};
    for (const lift of session.exercises) {
        liftMap[lift.name] = lift.sets * lift.reps * lift.weightLb;
    }
    return liftMap;
  }

  function mainLiftSummary(session: WorkoutSession): void {
    let mainLift: ExerciseEntry[] = session.exercises.filter(e => e.isMainLift);
    if (mainLift.length == 0) {
        console.log(`No lift marked as main lift`);
        return;
    }

    for (const lift of mainLift) {
        console.log(`MAIN LIFT: ${lift.name} sets: ${lift.sets} total reps: ${lift.sets * lift.reps} weight ${lift.weightLb}`)
    }
  }

  function main() {
  }

  main();