# Lesson 22b — Privacy & `pub`: private by default

*(Phase 6 — Organizing & generics, part 5. Lesson 22 answered "how do I name
it?" This lesson answers the other half: "am I ALLOWED to?" Everything is
private by default, `pub` opens one specific door — and the most instructive
trap in the phase is hiding right inside that sentence.)*

## 1. Why it exists

A module tree is only useful if you can decide what outsiders may touch.
Rust's stance: every item is **private to code outside its module** by
default, and you opt things open with `pub`.

Private-by-default is a feature, not a nuisance: anything you didn't mark
`pub` is yours to rename or rework freely, because no outside code could have
depended on it. (Much later, Lesson 32b turns this exact rule into an
API-design tool — hidden fields you're free to change forever. This is the
mechanism under it.)

## 2. The idea

- **Privacy points one way.** A **child** module can use its **ancestors'**
  private items (you saw `back_of_house` call the crate root's private
  `deliver_order` in Lesson 22); a **parent** *cannot* reach into a child's
  private items. Outward-in is blocked; inward-out is fine.
- **`pub` opens one specific item.** Not a family, not a subtree — the one
  item it's written on.
- **The surprise worth burning in: `pub` on a module does not make its
  contents public.** Marking `pub mod hosting` only lets outside code *refer
  to* `hosting`; each function inside still needs its own `pub`. Two strikes:
  the module *and* the item. Part 4 shows the error.
- There's also a middle setting, `pub(crate)` — public within this crate,
  hidden from crates that depend on yours. One sentence is all it needs now;
  **Book §7.3** and CR §27.4 have the fuller menu of scopes.

> **CR's one-line summary is worth memorising: modules are a privacy
> boundary.** Not files, not types — modules. (The type-shaped consequences
> of that are Lesson 22c.)

## 3. A tiny example to read — both doors open

The working version (library-shaped — no `main`): module `pub`, function
`pub`, and the path from Lesson 22 walks straight through:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

Count the `pub`s on the way in: one on `hosting` (so the path may *pass
through*), one on `add_to_waitlist` (so the call may *land*). Remove either
and the path is legal to write but not allowed to resolve — which is exactly
part 4.

(Why doesn't `front_of_house` need `pub` too? `eat_at_restaurant` is its
*sibling* — same module, the crate root — and privacy only bites from
*outside* the module. One-way rule, again.)

## 4. Common pitfalls / real compiler errors

**A public module with a private function — `E0603`.** Making the *module*
`pub` is not enough:

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}   // NOT pub
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

**Before you scroll — strike one is open (`pub mod`). Does the call land?**

```
error[E0603]: function `add_to_waitlist` is private
 --> main.rs:7:37
  |
7 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> main.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^
```

The module is reachable, but the function inside it isn't — exactly the
"`pub` on a module ≠ `pub` on its contents" surprise. The fix is `pub fn
add_to_waitlist`, and the compiler even points at the definition so you know
where to add it. The two matching exercises below are both faces of this
wall — a private module in a public path, and a private function in a public
module.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new visibility` works too.)* **Predict on
paper before each run.**

1. **The two-strike.** Build `mod garden { pub mod flowers { pub fn plant() {
   println!("planted"); } } }` and call it from `main` by full path. Then
   remove `pub` from `plant` (keep `pub mod`). **Predict the error code**,
   run to check, then restore it. Which word did the compiler use for the
   function?
2. **Strike the other match.** Restore `pub fn`, then remove `pub` from the
   *module* instead. **Predict**: same error code or different — and which
   name does the message point at this time?
3. **One-way street.** Put a *private* `fn secret_recipe()` at the crate
   root, and inside a module `kitchen` a `pub fn cook()` that calls
   `super::secret_recipe()`. **Predict**: does this compile — and would the
   reverse (main reaching a private fn *inside* `kitchen`) also work? Test
   both halves.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 22c: what `pub` means on a STRUCT — where the
answer is "less than you'd think" — and on an enum, where it's more.)*

## 6. What surprised you?

A sentence or two: did "`pub mod` doesn't publish its contents" catch you
out? Did the one-way rule (a child sees its parent's privates, never the
reverse) feel natural once you tested both directions? Tell me, and I'll tune
Lesson 22c to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§7.3**: exposing paths with
  `pub` and the two-strike `pub mod`/`pub fn` walkthrough (Listings 7-5 to
  7-7), plus the `pub(crate)` scope menu this lesson points at.
- **CR** — *Comprehensive Rust* (Google), §27.3–27.4: "modules are a privacy
  boundary" and `pub`/`pub(crate)`.
- Every snippet compiled (and the `main`-bearing ones run) on **rustc
  1.95.0**, edition 2024; `E0603` captured live (temp paths normalized to
  `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 22 — Paths: naming items across modules](22-paths.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 22c — pub on structs & enums →](22c-pub-structs-enums.md)
