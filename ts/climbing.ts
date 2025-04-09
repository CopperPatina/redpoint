import type { ClimbEntry, ClimbStyle, ClimbingSession, ClimbMetricsEntry } from "./types";
import { writeFileSync, readFileSync } from "fs";

  function printSentClimbs(session: ClimbingSession) {
    let sentClimbs: ClimbEntry[] = session.climbs.filter((c:ClimbEntry) => c.sent);
    for (const climb of sentClimbs) {
        console.log(`grade: ${climb.grade}, attempts: ${climb.attempts}, rests ${climb.rests ?? "unknown"}`);
    }
  }

  function averageRests(session: ClimbingSession): number {
    const restClimbs: ClimbEntry[] = session.climbs.filter(c => c.rests !== undefined);
  
    const totalRests = restClimbs.reduce((sum, climb) => sum + (climb.rests ?? 0), 0);
  
    return restClimbs.length === 0 ? 0 : totalRests / restClimbs.length;
  }

  export function saveClimbingLog(workout: ClimbingSession, file: string): void {
    const json = JSON.stringify(workout, null, 2);
    try {
        writeFileSync(file, json, "utf-8");
    } catch(error) {
        console.error(`Error saving file ${error}`);
    }
  }

  export function loadClimbingLog(file: string): ClimbingSession | null {
    try {
        let climbing = readFileSync(file, "utf-8");
        let session: ClimbingSession = JSON.parse(climbing);
        return session;
    } catch(error) {
        console.error(`Error loading file ${error}`);
        return null;
    }
  }

  export function saveClimbingMetricsLog(workout: ClimbMetricsEntry, file: string): void {
    const json = JSON.stringify(workout, null, 2);
    try {
        writeFileSync(file, json, "utf-8");
    } catch(error) {
        console.error(`Error saving file ${error}`);
    }
  }

  export function loadClimbingMetricsLog(file: string): ClimbMetricsEntry | null {
    try {
        let climbing = readFileSync(file, "utf-8");
        let session: ClimbMetricsEntry = JSON.parse(climbing);
        return session;
    } catch(error) {
        console.error(`Error loading file ${error}`);
        return null;
    }
  }