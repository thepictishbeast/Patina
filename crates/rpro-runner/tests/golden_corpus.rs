//! Golden-corpus integrity test — the corpus-wide invariants that the
//! per-exercise [`rpro_state::ExerciseMetadata::validate`] cannot see, because it
//! inspects one `.toml` in isolation.
//!
//! This is the in-tree, **toolchain-free** guard against curriculum drift:
//! - an exercise whose `id` area-prefix doesn't match the phase directory it
//!   lives in (the bug that stranded `move_out_of_borrow` in `09-advanced` and
//!   the collect-inference exercise before it),
//! - an exercise whose `difficulty` tier doesn't match its phase (the
//!   copy-pasted `difficulty = "advanced"` that sails past eye-review and every
//!   compile check),
//! - a duplicate `id` (silent wrong-lookup, since `select`/`current` find by id),
//! - a malformed `expected_error_code`,
//! - an empty `concept` (the hint ladder leans on it).
//!
//! Deliberately NOT here (kept where they belong, to keep this fast and pure):
//! compilation + `expected_error_code` *emission* live in
//! `scripts/verify-exercises.sh` (needs a real toolchain); book-anchor
//! resolution lives in `scripts/verify-book-anchors.mjs`.

use rpro_runner::discover;
use rpro_state::Difficulty;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The workspace `exercises/` directory, resolved relative to this crate so the
/// test is independent of the caller's CWD.
fn exercises_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../exercises")
}

/// The intended curriculum structure, phase by phase: each directory maps to the
/// set of `id` area-prefixes and the set of `difficulty` tiers permitted to live
/// there. The mapping is deliberately **not** mechanical — `07-generics-traits-
/// lifetimes` legitimately holds three areas, and `07b` spans two tiers as it
/// grows closures/smart-pointers — so it is spelled out here. Adding a phase, an
/// area, or a tier is a conscious edit to this table; that is the entire point,
/// because silent misplacement is exactly the drift this test exists to catch.
fn golden_map() -> Vec<(&'static str, &'static [&'static str], &'static [Difficulty])> {
    use Difficulty::{Advanced, Beginner, Intermediate};
    vec![
        ("01-basics", &["basics"], &[Beginner]),
        ("02-control-flow", &["control-flow"], &[Beginner]),
        ("03-text-and-collections", &["collections"], &[Beginner]),
        ("04-ownership", &["ownership"], &[Beginner]),
        ("05-types-and-matching", &["types"], &[Beginner]),
        ("06-modules", &["modules"], &[Beginner]),
        (
            "07-generics-traits-lifetimes",
            &["generics", "traits", "lifetimes"],
            &[Intermediate],
        ),
        (
            "07b-functional-and-smart-pointers",
            &["iterators", "closures", "smart-pointers"],
            &[Intermediate, Advanced],
        ),
        ("08-concurrency", &["concurrency"], &[Advanced]),
        ("09-advanced", &["advanced"], &[Advanced]),
    ]
}

/// A well-formed rustc error code: `E` followed by exactly four digits.
fn is_error_code(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 5 && b[0] == b'E' && b[1..].iter().all(u8::is_ascii_digit)
}

/// The phase directory an exercise physically sits in (`exercises/<phase>/x.rs`
/// → `<phase>`).
fn phase_dir(source: &Path, root: &Path) -> String {
    source
        .strip_prefix(root)
        .ok()
        .and_then(|rel| rel.components().next())
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .unwrap_or_default()
}

#[test]
fn golden_corpus_holds_its_invariants() {
    let root = exercises_root();
    let exercises = discover(&root).expect("the real corpus must discover without error");
    // A wrong path yields an empty walk (Ok(vec![])); the floor is the signal.
    assert!(
        !exercises.is_empty(),
        "no exercises discovered under {} — wrong path or empty corpus",
        root.display()
    );

    let map = golden_map();
    let mut seen_ids = BTreeSet::new();

    for ex in &exercises {
        let id = &ex.meta.id;

        // 1. ids are unique across the whole corpus (find-by-id must be 1:1).
        assert!(seen_ids.insert(id.clone()), "duplicate exercise id: {id}");

        // 2. concept is present (validate() doesn't check it; the hint ladder uses it).
        assert!(
            !ex.meta.concept.trim().is_empty(),
            "{id}: empty concept tag"
        );

        // 3. expected_error_code, when present, is E#### shaped.
        if let Some(code) = &ex.meta.expected_error_code {
            assert!(
                is_error_code(code),
                "{id}: malformed expected_error_code {code:?} (want E####)"
            );
        }

        // 4. the exercise sits in a known phase, and its id-area + difficulty are
        //    both allowed there (this is the curriculum-drift guard).
        let phase = phase_dir(&ex.source, &root);
        let entry = map
            .iter()
            .find(|(dir, _, _)| *dir == phase)
            .unwrap_or_else(|| {
                panic!("{id}: phase dir {phase:?} is not in the golden map — add it deliberately")
            });
        let area = id.split('/').next().unwrap_or_default();
        assert!(
            entry.1.contains(&area),
            "{id}: area {area:?} is not allowed in {phase:?} (allowed: {:?})",
            entry.1
        );
        assert!(
            entry.2.contains(&ex.meta.difficulty),
            "{id}: difficulty {:?} is not allowed in {phase:?} (allowed: {:?})",
            ex.meta.difficulty,
            entry.2
        );
    }
}
