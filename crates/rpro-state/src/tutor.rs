//! Tutor scaffolding — the *guide-not-solve* contract, made executable.
//!
//! This is the **mechanism** of tutoring from `docs/EDUCATION.md`, not lesson
//! content: it is lesson-agnostic (so it doesn't trip the "show the matrix
//! before lessons" gate) and language-agnostic (it speaks the neutral seam —
//! never names a toolchain). A future pluggable AI-tutor backend prepends
//! [`GUARDRAILS`] to its system prompt and asks this module what to do at each
//! [`LoopStep`] and which [`HintRung`] to climb.
//!
//! ## What this enforces — and what it can't
//!
//! Everything here is **meta-instruction to the tutor**, never a fixed
//! learner-facing string and never a worked fix. That keeps it both pedagogically
//! correct (the worked example at the top rung must be generated fresh for the
//! observed error, never the learner's own code) and seam-clean (no error-code
//! or toolchain literals live in these templates).
//!
//! This is **scaffolding-level** enforcement: it guarantees the *instructions*
//! the tutor receives forbid revealing the fix. It does **not** and cannot
//! guarantee an LLM backend obeys them — that is the backend's responsibility and
//! is not verifiable here. The tests assert the contract's *structure*, not a
//! model's behaviour.

/// Who primarily acts at a loop step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    /// The learner produces something (a prediction, a diagnosis, an edit).
    Learner,
    /// The app drives the real toolchain or shows raw output / a comparison.
    App,
    /// The tutor speaks — bound by [`GUARDRAILS`].
    Tutor,
}

/// The nine steps of the interactive loop (`docs/EDUCATION.md`). Seven are
/// tutor-silent by invariant; the tutor only truly acts at [`LoopStep::Guide`]
/// (with a light supporting role at [`LoopStep::Explain`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopStep {
    /// Capture + lock a typed prediction before any run.
    Predict,
    /// App runs the real toolchain.
    Run,
    /// Show prediction beside the real result; hit/miss on compiled? + code.
    Compare,
    /// Display the complete, unedited compiler output.
    ReadRaw,
    /// Learner diagnoses the error in their own words — tutor silent.
    Diagnose,
    /// Tutor climbs the hint ladder, lowest rung first.
    Guide,
    /// Learner asks for the official explanation; tutor ties it to their line.
    Explain,
    /// Learner edits + re-predicts, then re-runs (loop closes on a clean run).
    Retry,
    /// The error code enters the spaced-repetition deck.
    Recall,
}

impl LoopStep {
    /// All nine steps in loop order.
    #[must_use]
    pub const fn all() -> [Self; 9] {
        [
            Self::Predict,
            Self::Run,
            Self::Compare,
            Self::ReadRaw,
            Self::Diagnose,
            Self::Guide,
            Self::Explain,
            Self::Retry,
            Self::Recall,
        ]
    }

    /// Short display label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Predict => "PREDICT",
            Self::Run => "RUN",
            Self::Compare => "COMPARE",
            Self::ReadRaw => "READ-RAW",
            Self::Diagnose => "DIAGNOSE",
            Self::Guide => "GUIDE",
            Self::Explain => "EXPLAIN",
            Self::Retry => "RETRY",
            Self::Recall => "RECALL",
        }
    }

    /// Who primarily acts at this step.
    #[must_use]
    pub const fn actor(self) -> Actor {
        match self {
            Self::Predict | Self::Diagnose | Self::Retry => Actor::Learner,
            Self::Run | Self::Compare | Self::ReadRaw | Self::Recall => Actor::App,
            Self::Guide | Self::Explain => Actor::Tutor,
        }
    }

    /// Whether the tutor speaks at this step. Only [`LoopStep::Guide`] is the
    /// active tutoring turn; [`LoopStep::Explain`] is a light supporting role.
    #[must_use]
    pub const fn is_tutor_turn(self) -> bool {
        matches!(self, Self::Guide)
    }

    /// The tutor's one-line directive at this step. For tutor-silent steps this
    /// is an explicit "stay out of the way" instruction — the invariant that the
    /// learner must produce the prediction/diagnosis/edit unaided is load-bearing.
    #[must_use]
    pub const fn tutor_directive(self) -> &'static str {
        match self {
            Self::Predict => {
                "Stay silent. The learner locks a typed prediction (compiles? which \
                 error code? one-word fix?) before anything runs — do not hint."
            }
            Self::Run => "Stay silent. The app runs the real toolchain; never simulate output.",
            Self::Compare => {
                "Stay silent. The app marks the prediction hit/miss on (a) compiled and \
                 (b) the exact error code — the unit of correctness is the code, not vibes."
            }
            Self::ReadRaw => {
                "Stay silent. The learner reads the complete, unedited compiler output — \
                 never paraphrase or truncate it."
            }
            Self::Diagnose => {
                "Stay silent. The learner says, in their own words, what the error means, \
                 which line it blames, and what fix it suggests — before you speak."
            }
            Self::Guide => {
                "Your turn. Confirm what they got right, then climb ONE hint rung (lowest \
                 untried first). Never name the token to type; never post corrected code."
            }
            Self::Explain => {
                "Light touch. The learner asks the explain tool for the official description \
                 of the error code; you only connect that official text back to their line."
            }
            Self::Retry => {
                "Stay silent. The learner edits, re-predicts, and re-runs. The loop closes \
                 only when their prediction matches a real clean run."
            }
            Self::Recall => {
                "Stay silent. The app schedules the error code for spaced review (sooner if \
                 the prediction was missed)."
            }
        }
    }
}

/// The four rungs of the hint ladder (`docs/EDUCATION.md`). The tutor climbs
/// **one rung per turn**, lowest first, escalating only on a genuine
/// stuck-after-attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintRung {
    /// Orient, reveal nothing — aim them at the raw output they already have.
    Nudge,
    /// A probing question whose answer *is* the next step; they must produce it.
    ProbingQuestion,
    /// Name the underlying rule + point at the authoritative reference. No token.
    ConceptPointer,
    /// Only if still stuck: work a *different* example partway, never their code.
    PartialWorkedAnalogy,
}

impl HintRung {
    /// All four rungs, lowest first.
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::Nudge, Self::ProbingQuestion, Self::ConceptPointer, Self::PartialWorkedAnalogy]
    }

    /// Rung number (1-based, lowest = 1).
    #[must_use]
    pub const fn level(self) -> u8 {
        match self {
            Self::Nudge => 1,
            Self::ProbingQuestion => 2,
            Self::ConceptPointer => 3,
            Self::PartialWorkedAnalogy => 4,
        }
    }
}

/// What the tutor knows when it speaks. No solution / fix is ever an input —
/// the tutor cannot reveal what it is never given.
#[derive(Debug, Clone, Copy)]
pub struct TutorContext<'a> {
    /// The current exercise's concept tag (e.g. `"mutability"`).
    pub concept: &'a str,
    /// The error code the learner actually hit, if the run produced one.
    pub observed_code: Option<&'a str>,
    /// Attempts so far on this exercise — the ladder may climb higher the more
    /// they have genuinely tried.
    pub attempts: u32,
}

impl TutorContext<'_> {
    /// How the tutor should refer to the error in prose — the observed code if
    /// there is one, else the neutral phrase.
    fn code_phrase(&self) -> String {
        self.observed_code
            .map_or_else(|| "the error code".to_string(), |c| format!("error {c}"))
    }
}

/// The hard guardrails every tutor turn must obey (`docs/EDUCATION.md`
/// §"How the AI tutor must GUIDE not solve"). A backend prepends these to its
/// system prompt verbatim. Neutral by design — no toolchain or error-code
/// literals — so the rules travel across surfaces unchanged.
pub const GUARDRAILS: &[&str] = &[
    "Never post corrected code for their exercise. A different teaching snippet to \
     illustrate a mechanism is fine; their practice code never is.",
    "Never name the exact token to type until they have attempted it. Climb the ladder \
     one rung; do not skip to the answer even when asked outright.",
    "Never paraphrase or invent compiler output. Point them at the raw output already \
     shown and at the official explanation of the error code.",
    "No cross-language analogies. Explain mechanisms from the everyday world or from the \
     language itself.",
    "Phone-concise: one nudge per turn, short. Never dump the whole ladder at once.",
    "Default to a question. On 'I'm stuck', ask what they predicted and what the error's \
     location line points at — do not explain.",
    "Confirm-then-nudge: acknowledge the correct part of their reading before nudging the \
     gap, every turn.",
];

/// The meta-instruction the tutor backend executes to produce a single GUIDE turn
/// at `rung` for `ctx`. This is an instruction *to the tutor*, parameterised by
/// the concept and observed code — never a fixed reply, and never a worked fix.
/// The top rung is itself an instruction to *generate* a throwaway example, so no
/// concrete fix is ever stored here.
#[must_use]
pub fn guide_rung(rung: HintRung, ctx: &TutorContext) -> String {
    let code = ctx.code_phrase();
    match rung {
        HintRung::Nudge => format!(
            "Rung 1 (NUDGE) — reveal nothing. Point them at the raw output they already have: \
             ask them to read aloud the line the compiler underlines, or to tell you {code} at \
             the top. Confirm any correct reading first."
        ),
        HintRung::ProbingQuestion => format!(
            "Rung 2 (PROBING QUESTION) — still no content. Ask ONE question about {concept} whose \
             answer is the next step and which they must produce themselves (e.g. point them at \
             the `help:` block and ask what differs from the line they wrote).",
            concept = ctx.concept
        ),
        HintRung::ConceptPointer => format!(
            "Rung 3 (CONCEPT POINTER) — name the rule, not the fix. Name the underlying idea \
             behind {concept} and tell them to ask the explain tool for the official description \
             of {code}, then report back the fix it names. Do not name the token to type.",
            concept = ctx.concept
        ),
        HintRung::PartialWorkedAnalogy => {
            "Rung 4 (PARTIAL WORKED ANALOGY) — only if still stuck after attempting. GENERATE a \
             DIFFERENT throwaway snippet that hits the same error, worked to one step short of the \
             fix, and ask them to apply the equivalent edit to their own line. Never touch their \
             exercise; everyday-world framing only, no cross-language analogy. Stop here — do not \
             also rewrite their code."
                .to_string()
        }
    }
}

/// Assemble the full GUIDE-turn instruction a backend would act on: the standing
/// guardrails, then the rung-specific meta-instruction. Takes no solution input,
/// so by construction it carries no fix.
#[must_use]
pub fn guide_turn(ctx: &TutorContext, rung: HintRung) -> String {
    let mut s = String::from("You are the tutor: guide, never solve. Standing rules:\n");
    for rule in GUARDRAILS {
        s.push_str("- ");
        s.push_str(rule);
        s.push('\n');
    }
    s.push_str("\nThis turn:\n");
    s.push_str(&guide_rung(rung, ctx));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> TutorContext<'static> {
        TutorContext { concept: "mutability", observed_code: Some("E4321"), attempts: 1 }
    }

    #[test]
    fn loop_has_nine_steps_in_order() {
        let all = LoopStep::all();
        assert_eq!(all.len(), 9);
        assert_eq!(all[0], LoopStep::Predict);
        assert_eq!(all[8], LoopStep::Recall);
        // Labels are unique.
        let mut labels: Vec<&str> = all.iter().map(|s| s.label()).collect();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), 9, "every step has a distinct label");
    }

    #[test]
    fn only_guide_is_the_tutor_turn() {
        // The doc invariant: the tutor is silent everywhere but GUIDE.
        for step in LoopStep::all() {
            assert_eq!(
                step.is_tutor_turn(),
                step == LoopStep::Guide,
                "{} tutor-turn flag wrong",
                step.label()
            );
        }
        // Diagnose is explicitly the learner's unaided turn.
        assert_eq!(LoopStep::Diagnose.actor(), Actor::Learner);
        assert!(LoopStep::Diagnose.tutor_directive().contains("Stay silent"));
    }

    #[test]
    fn guardrails_carry_the_load_bearing_rules() {
        let joined = GUARDRAILS.join(" ").to_lowercase();
        assert!(joined.contains("never post corrected code"), "no-corrected-code rule missing");
        assert!(joined.contains("no cross-language analog"), "no-analogy rule missing");
        assert!(joined.contains("one nudge per turn"), "phone-concise rule missing");
        assert!(joined.contains("confirm"), "confirm-then-nudge rule missing");
    }

    #[test]
    fn rungs_climb_one_at_a_time_and_only_top_works_an_example() {
        let rungs = HintRung::all();
        assert_eq!(rungs.len(), 4);
        for (i, r) in rungs.iter().enumerate() {
            assert_eq!(r.level() as usize, i + 1, "rung levels are 1..=4 in order");
        }
        // Lower rungs reveal no worked example; only rung 4 generates one, and even
        // then on a DIFFERENT snippet — never the learner's exercise.
        let nudge = guide_rung(HintRung::Nudge, &ctx());
        assert!(nudge.contains("reveal nothing"));
        let top = guide_rung(HintRung::PartialWorkedAnalogy, &ctx());
        assert!(top.contains("DIFFERENT") && top.contains("Never touch their exercise"));
        assert!(top.contains("GENERATE"), "the worked example is generated, not a stored fix");
    }

    #[test]
    fn guide_turn_prepends_guardrails_and_names_no_fix() {
        let s = guide_turn(&ctx(), HintRung::ConceptPointer);
        assert!(s.contains("guide, never solve"));
        assert!(s.contains("Never post corrected code"));
        // Concept + observed code are interpolated; the structure carries no fix
        // because none is ever passed in.
        assert!(s.contains("mutability"));
        assert!(s.contains("error E4321"));
    }
}
