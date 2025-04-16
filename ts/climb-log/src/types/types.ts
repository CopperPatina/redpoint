export type ClimbStyle = "boulder" | "rope";

export interface ClimbEntry {
  name?: string;
  grade: string;
  attempts: number;
  sent: boolean;
  reached_top: boolean;
  lead: boolean;
  rests?: number;
}

export interface ClimbingSession {
  date: string;
  location: string;
  style: ClimbStyle;
  notes?: string;
  climbs: ClimbEntry[];
}

export interface ClimbMetricsEntry {
  date: string;
  fingerStrengthPercentBW?: number;
  maxPullupPercentBW?: number;
  notes?: string;
}

export interface ExerciseEntry {
  name: string;
  sets: number;
  reps: number;
  weightLb: number;
  rpe?: number;
  isMainLift?: boolean;
}

export interface WorkoutSession {
  date: string;
  notes?: string;
  exercises: ExerciseEntry[];
}