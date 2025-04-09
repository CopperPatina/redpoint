"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = require("fs");
const w = {
    date: "04-06-2025",
    exercises: [
        { name: "deadlift", sets: 4, reps: 3, weightLb: 220, isMainLift: true },
        { name: "squat", sets: 3, reps: 10, weightLb: 150, isMainLift: false },
        { name: "bulgarian split squat", sets: 3, reps: 8, weightLb: 50, isMainLift: false }
    ]
};
function printSentClimbs(session) {
    let sentClimbs = session.climbs.filter((c) => c.sent);
    for (const climb of sentClimbs) {
        console.log(`grade: ${climb.grade}, attempts: ${climb.attempts}, rests ${climb.rests ?? "unknown"}`);
    }
}
function averageRests(session) {
    const restClimbs = session.climbs.filter(c => c.rests !== undefined);
    const totalRests = restClimbs.reduce((sum, climb) => sum + (climb.rests ?? 0), 0);
    return restClimbs.length === 0 ? 0 : totalRests / restClimbs.length;
}
function totalVolume(session) {
    let sum = 0;
    for (const exercise of session.exercises) {
        sum += exercise.reps * exercise.sets * exercise.weightLb;
    }
    return sum;
}
function volumeByLift(session) {
    let liftMap = {};
    for (const lift of session.exercises) {
        liftMap[lift.name] = lift.sets * lift.reps * lift.weightLb;
    }
    return liftMap;
}
function mainLiftSummary(session) {
    let mainLift = session.exercises.filter(e => e.isMainLift);
    if (mainLift.length == 0) {
        console.log(`No lift marked as main lift`);
        return;
    }
    for (const lift of mainLift) {
        console.log(`MAIN LIFT: ${lift.name} sets: ${lift.sets} total reps: ${lift.sets * lift.reps} weight ${lift.weightLb}`);
    }
}
function saveWorkoutLog(workout, file) {
    const json = JSON.stringify(workout, null, 2);
    try {
        (0, fs_1.writeFileSync)(file, json, "utf-8");
    }
    catch (error) {
        console.error(`Error saving file ${error}`);
    }
}
function loadWorkoutLog(file) {
    try {
        let workout = (0, fs_1.readFileSync)(file, "utf-8");
        let session = JSON.parse(workout);
        return session;
    }
    catch (error) {
        console.error(`Error loading file ${error}`);
        return null;
    }
}
function workoutFileName(workout) {
    return "workout-" + workout.date;
}
function main() {
    let filePath = "./logs/" + workoutFileName(w);
    saveWorkoutLog(w, filePath);
    let wo = loadWorkoutLog(filePath);
    if (wo) {
        console.log(`Workout on ${wo.date}`);
        for (const e of wo.exercises) {
            console.log(`- ${e.name}: ${e.sets}x${e.reps} @ ${e.weightLb}lb`);
        }
    }
}
main();
