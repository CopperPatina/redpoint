"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs_1 = require("fs");
var w = {
    date: "04-06-2025",
    exercises: [
        { name: "deadlift", sets: 4, reps: 3, weightLb: 220, isMainLift: true },
        { name: "squat", sets: 3, reps: 10, weightLb: 150, isMainLift: false },
        { name: "bulgarian split squat", sets: 3, reps: 8, weightLb: 50, isMainLift: false }
    ]
};
function printSentClimbs(session) {
    var _a;
    var sentClimbs = session.climbs.filter(function (c) { return c.sent; });
    for (var _i = 0, sentClimbs_1 = sentClimbs; _i < sentClimbs_1.length; _i++) {
        var climb = sentClimbs_1[_i];
        console.log("grade: ".concat(climb.grade, ", attempts: ").concat(climb.attempts, ", rests ").concat((_a = climb.rests) !== null && _a !== void 0 ? _a : "unknown"));
    }
}
function averageRests(session) {
    var restClimbs = session.climbs.filter(function (c) { return c.rests !== undefined; });
    var totalRests = restClimbs.reduce(function (sum, climb) { var _a; return sum + ((_a = climb.rests) !== null && _a !== void 0 ? _a : 0); }, 0);
    return restClimbs.length === 0 ? 0 : totalRests / restClimbs.length;
}
function totalVolume(session) {
    var sum = 0;
    for (var _i = 0, _a = session.exercises; _i < _a.length; _i++) {
        var exercise = _a[_i];
        sum += exercise.reps * exercise.sets * exercise.weightLb;
    }
    return sum;
}
function volumeByLift(session) {
    var liftMap = {};
    for (var _i = 0, _a = session.exercises; _i < _a.length; _i++) {
        var lift = _a[_i];
        liftMap[lift.name] = lift.sets * lift.reps * lift.weightLb;
    }
    return liftMap;
}
function mainLiftSummary(session) {
    var mainLift = session.exercises.filter(function (e) { return e.isMainLift; });
    if (mainLift.length == 0) {
        console.log("No lift marked as main lift");
        return;
    }
    for (var _i = 0, mainLift_1 = mainLift; _i < mainLift_1.length; _i++) {
        var lift = mainLift_1[_i];
        console.log("MAIN LIFT: ".concat(lift.name, " sets: ").concat(lift.sets, " total reps: ").concat(lift.sets * lift.reps, " weight ").concat(lift.weightLb));
    }
}
function saveWorkoutLog(workout, file) {
    var json = JSON.stringify(workout, null, 2);
    try {
        (0, fs_1.writeFileSync)(file, json, "utf-8");
    }
    catch (error) {
        console.error("Error saving file {error}");
    }
}
function loadWorkoutLog(file) {
    try {
        var workout = (0, fs_1.readFileSync)(file);
        var session = JSON.parse(workout);
        return session;
    }
    catch (error) {
        console.error("Error loading file {error}");
        return null;
    }
}
function workoutFileName(workout) {
    return "workout-" + workout.date;
}
function main() {
    var filePath = "./logs" + workoutFileName(w);
    saveWorkoutLog(w, filePath);
    var wo = loadWorkoutLog(filePath);
    console.log("{wo}");
}
main();
