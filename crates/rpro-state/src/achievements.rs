//! Achievements / badges — threshold-unlocked milestones (board #36–41).
//!
//! A pure, side-effect-free model: running activity tallies (`Counters`) plus a
//! set of `Badge`s, each unlocked when a counter crosses its threshold. No I/O —
//! keep the tallies wherever the progress store lives and ask this module which
//! badges are earned, or which just unlocked.
//!
//! The counter→threshold→unlock shape is adapted (not copied) from Paul's
//! claude-buddy companion; the badges themselves are re-themed for learning Rust.

use serde::{Deserialize, Serialize};

/// Running tallies of what a learner has done — the inputs badges unlock from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Counters {
    /// Exercises compiled/ran successfully.
    pub exercises_solved: u32,
    /// Compiler errors correctly predicted before running.
    pub predictions_correct: u32,
    /// Exercises solved without opening a hint.
    pub hintless_solves: u32,
    /// Lessons/sections finished end to end.
    pub tracks_completed: u32,
    /// Longest run of consecutive active days.
    pub max_streak_days: u32,
}

/// A milestone a learner can unlock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Badge {
    /// First exercise ever solved.
    FirstSolve,
    /// Ten exercises solved.
    TenSolved,
    /// Fifty exercises solved.
    FiftySolved,
    /// A hundred exercises solved.
    CenturySolved,
    /// Twenty-five compiler errors predicted correctly.
    ErrorOracle,
    /// Ten exercises solved with no hints.
    HintlessHero,
    /// First lesson/track completed.
    TrackTamer,
    /// Five lessons/tracks completed.
    Marathoner,
    /// Practised three days in a row.
    ThreeDayStreak,
    /// Practised seven days in a row.
    WeekStreak,
}

impl Badge {
    /// Every badge, for iteration.
    pub const ALL: [Badge; 10] = [
        Badge::FirstSolve,
        Badge::TenSolved,
        Badge::FiftySolved,
        Badge::CenturySolved,
        Badge::ErrorOracle,
        Badge::HintlessHero,
        Badge::TrackTamer,
        Badge::Marathoner,
        Badge::ThreeDayStreak,
        Badge::WeekStreak,
    ];

    /// True once `counters` satisfy this badge's unlock condition.
    pub fn earned_by(self, counters: &Counters) -> bool {
        match self {
            Badge::FirstSolve => counters.exercises_solved >= 1,
            Badge::TenSolved => counters.exercises_solved >= 10,
            Badge::FiftySolved => counters.exercises_solved >= 50,
            Badge::CenturySolved => counters.exercises_solved >= 100,
            Badge::ErrorOracle => counters.predictions_correct >= 25,
            Badge::HintlessHero => counters.hintless_solves >= 10,
            Badge::TrackTamer => counters.tracks_completed >= 1,
            Badge::Marathoner => counters.tracks_completed >= 5,
            Badge::ThreeDayStreak => counters.max_streak_days >= 3,
            Badge::WeekStreak => counters.max_streak_days >= 7,
        }
    }

    /// Short display name.
    pub const fn title(self) -> &'static str {
        match self {
            Badge::FirstSolve => "First Blood",
            Badge::TenSolved => "Getting the Hang of It",
            Badge::FiftySolved => "Half-Century",
            Badge::CenturySolved => "Centurion",
            Badge::ErrorOracle => "Error Oracle",
            Badge::HintlessHero => "Hintless Hero",
            Badge::TrackTamer => "Track Tamer",
            Badge::Marathoner => "Marathoner",
            Badge::ThreeDayStreak => "On a Roll",
            Badge::WeekStreak => "Seven-Day Streak",
        }
    }

    /// One-line description of how it's earned.
    pub const fn description(self) -> &'static str {
        match self {
            Badge::FirstSolve => "Solve your first exercise.",
            Badge::TenSolved => "Solve ten exercises.",
            Badge::FiftySolved => "Solve fifty exercises.",
            Badge::CenturySolved => "Solve a hundred exercises.",
            Badge::ErrorOracle => "Predict twenty-five compiler errors correctly.",
            Badge::HintlessHero => "Solve ten exercises without a hint.",
            Badge::TrackTamer => "Finish a whole lesson track.",
            Badge::Marathoner => "Finish five lesson tracks.",
            Badge::ThreeDayStreak => "Practise three days in a row.",
            Badge::WeekStreak => "Practise seven days in a row.",
        }
    }

    /// Bonus XP awarded the moment this badge unlocks.
    pub const fn xp_reward(self) -> u64 {
        match self {
            Badge::FirstSolve => 25,
            Badge::TenSolved => 50,
            Badge::FiftySolved => 100,
            Badge::CenturySolved => 200,
            Badge::ErrorOracle => 75,
            Badge::HintlessHero => 80,
            Badge::TrackTamer => 40,
            Badge::Marathoner => 150,
            Badge::ThreeDayStreak => 30,
            Badge::WeekStreak => 60,
        }
    }
}

/// Every badge currently satisfied by `counters`.
pub fn earned(counters: &Counters) -> Vec<Badge> {
    Badge::ALL
        .iter()
        .copied()
        .filter(|b| b.earned_by(counters))
        .collect()
}

/// Badges that unlock crossing from `before` to `after` — the ones worth
/// celebrating right now (earned by `after` but not yet by `before`).
pub fn newly_unlocked(before: &Counters, after: &Counters) -> Vec<Badge> {
    Badge::ALL
        .iter()
        .copied()
        .filter(|b| b.earned_by(after) && !b.earned_by(before))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_earned_at_zero() {
        assert!(earned(&Counters::default()).is_empty());
    }

    #[test]
    fn solve_thresholds_are_exact() {
        let mut c = Counters {
            exercises_solved: 9,
            ..Counters::default()
        };
        assert!(!Badge::TenSolved.earned_by(&c));
        c.exercises_solved = 10;
        assert!(Badge::TenSolved.earned_by(&c));
        assert!(Badge::FirstSolve.earned_by(&c));
        assert!(!Badge::FiftySolved.earned_by(&c));
    }

    #[test]
    fn newly_unlocked_returns_only_the_delta() {
        let before = Counters {
            exercises_solved: 9,
            ..Counters::default()
        };
        let after = Counters {
            exercises_solved: 10,
            ..Counters::default()
        };
        let fresh = newly_unlocked(&before, &after);
        // FirstSolve was already earned at 9; only TenSolved is new.
        assert_eq!(fresh, vec![Badge::TenSolved]);
    }

    #[test]
    fn no_regression_and_no_dup() {
        // Same counters twice → nothing "newly" unlocks.
        let c = Counters {
            exercises_solved: 100,
            predictions_correct: 25,
            hintless_solves: 10,
            tracks_completed: 5,
            max_streak_days: 7,
        };
        assert!(newly_unlocked(&c, &c).is_empty());
        // ...and with everything maxed, all ten badges are earned.
        assert_eq!(earned(&c).len(), Badge::ALL.len());
    }

    #[test]
    fn streak_and_track_badges() {
        let c = Counters {
            max_streak_days: 7,
            tracks_completed: 5,
            ..Counters::default()
        };
        assert!(Badge::ThreeDayStreak.earned_by(&c));
        assert!(Badge::WeekStreak.earned_by(&c));
        assert!(Badge::TrackTamer.earned_by(&c));
        assert!(Badge::Marathoner.earned_by(&c));
        // But solving-based badges stay locked.
        assert!(!Badge::FirstSolve.earned_by(&c));
    }

    #[test]
    fn every_badge_has_metadata_and_reward() {
        for b in Badge::ALL {
            assert!(!b.title().is_empty());
            assert!(!b.description().is_empty());
            assert!(b.xp_reward() > 0, "{b:?} has no reward");
        }
    }
}
