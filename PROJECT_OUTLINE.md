# Week 2 — Lab 2: Config Parser CLI (20 pts)

## Module Name & Duration
**Stage/Module:** Systems Programming with Rust — Configuration & CLIs  
**Duration:** 1 week (Mon–Fri)

### Unit: Config Parser CLI

---

## Learning Objectives
- **Implement** deserialization/serialization of TOML/JSON configs using `serde` and format-specific crates.  
- **Analyze** trade-offs in config representation (owned vs. borrowed data; lifetimes vs. simplicity) to keep APIs ergonomic.  
- **Design** a modular Rust crate with clean separation between CLI parsing, config I/O, and mutation logic.  
- **Evaluate** CLI UX with clear, defensive error messages and helpful diagnostics for invalid keys/types.  
- **Create** a robust “edit” command that round-trips values while preserving data types.

---

## Lecture Themes
1. **Config as Data Models**  
   Mapping TOML/JSON to Rust structs/enums; when to use `Value`-style dynamic trees vs. strongly typed models; implications for round-tripping and extensibility.  

2. **Serde in Practice**  
   Derives (`Serialize`, `Deserialize`), custom de/ser for tricky fields, and format adapters (`serde_json`, `toml`). Owned data for simpler lifetimes vs. `Cow`/smart refs.  

3. **CLI Ergonomics & UX**  
   Command design (`read`, `set`, `delete`, `list`), flags and subcommands (with `clap`), error messaging patterns, exit codes, and idempotent behavior.  

4. **Testing Strategies**  
   Golden files, unit tests for path parsing and type validation, and property-style tests for round-trip invariants.  

---

## Core Readings (Mon–Tue)
- **The Rust Programming Language (TRPL)**  
  - Ch. 5 **Structs**, Ch. 6 **Enums & Pattern Matching**, Ch. 7 **Modules**, Ch. 8 **Common Collections** (focus on `HashMap`/`Vec` for dynamic trees).
- **Serde**  
  - *Serde Quickstart* and *Data Model* sections; examples on custom `Deserialize`.  
- **TOML & JSON crates**  
  - `toml` crate docs: value model & `from_str`/`to_string_pretty`.  
  - `serde_json` crate docs: `Value`, pointer APIs, and number handling.  
- **CLI Framework**  
  - `clap` (v4) *Getting Started* & *Subcommands*.  

*(Offline docs or local examples are fine.)*

---

## Discussion Questions (Wed)
1. **Dynamic vs. Strongly Typed**  
   When is a `serde_json::Value`/`toml::Value` preferable to a dedicated struct? What does each choice mean for validation and evolution?  

2. **Key-Path Semantics**  
   How should key paths handle arrays and nested tables? What error messages best guide users when paths don’t exist?  

3. **Round-Trip Guarantees**  
   What does it truly mean to “preserve types”? Where can round-trip fidelity break (e.g., number representation, ordering), and how do we test it?  

4. **Ergonomics vs. Safety**  
   Which defaults (dry-run, confirmation, backups) improve UX without hiding errors?  

---

## Practical Exercises (Wed–Thu)
- **E1 – Parse & Print:** Load a TOML file into a dynamic value tree and pretty-print it as TOML and JSON.  
- **E2 – Path Parser:** Implement `risk.max_order_size`-style key-path parsing with array indices (e.g., `venues[2].symbol`).  
- **E3 – Mutations:** Implement `get/set/delete` by path; return structured errors for missing keys or type mismatches.  
- **E4 – Round-Trip:** Read → mutate → write → read → assert equality (type-preserving).  
- **E5 (Optional, Advanced):** Implement `list` to show subtree contents with types and paths.  

---

## Project: Config Parser CLI

### Scope
Build a Rust CLI that edits configuration files in TOML and/or JSON. Users can `read`, `set`, `delete`, and `list` values using dotted key paths (with array indices). The tool must deserialize, mutate, and serialize while **preserving data types** through a round-trip. Emphasis on modular design and excellent CLI ergonomics.

### Deliverables
- `/config/cli/` (Rust crate) — main binary & library modules (I/O, path, ops, cli).  
- `/config/cli/tests/` — unit tests (path parsing, mutations, round-trip) and integration tests.  
- `/config/cli/samples/` — sample input configs + golden expected outputs.  
- `/config/cli/README.md` — usage, design overview, assumptions, and examples.  
- *(Optional)* **Schema assets** and ENV/overlay examples if you attempt stretch goals.  

### Required Commands (examples)
- `config read --file settings.toml --get risk.max_order_size`  
- `config set --file settings.toml --path risk.max_order_size --value 1000`  
- `config delete --file settings.toml --path risk.limits[1]`  
- `config list --file settings.toml --path risk`  

### Behavioral Requirements
- Support: `read`, `set`, `delete`, `list`.  
- Round-trip preserving **types** (e.g., integers stay integers; booleans stay booleans).  
- Defensive errors: bad paths, invalid indices, incompatible types, parse failures, and write failures with clear messages & non-zero exit codes.  

### Testing & Docs (Fri)
- **Goldens:** sample files → exact expected files after operations.  
- **Unit Tests:** path parser, mutation functions, round-trip invariants.  
- **README:** CLI usage table, error catalog, and design notes.  

### Stretch Goals (Optional)
- **Schema validation:** validate files/changes against a schema (e.g., custom validators).  
- **Default overlay/precedence:** support layered sources *(ENV > CLI > File as specified)* with clear precedence and explain conflicts.  

---

## Evaluation Criteria (20 pts)
- **Parsing & Round-Trip (6 pts):** Correct load/mutate/save across TOML/JSON; numbers/bools/strings keep types; robust handling of nested paths and arrays.  
- **CLI UX & Errors (5 pts):** Clear subcommands and flags; readable help; explicit, actionable error messages; proper exit codes.  
- **Design & Modularity (4 pts):** Separation of concerns (cli ↔ core ops ↔ I/O ↔ path); small, testable functions; minimal unsafe.  
- **Tests & Docs (5 pts):** Golden and unit tests cover core paths and edge cases; README with examples & rationale.  

---

## Excellence Indicators
- Supports **both TOML and JSON** seamlessly with automatic detection and consistent semantics.  
- **Helpful diagnostics** that suggest likely correct paths or show nearest existing keys.  
- **Atomic writes/backups** to avoid corruption (write to temp + rename).  
- **Dry-run/`--check` mode** with diff-like output for `set/delete`.  
- **Extensible path grammar** (quoted segments, wildcard-safe parsing) with thorough tests.  
- **Developer experience**: clear module boundaries, doc comments, and examples in `--help` and README.  

---

## Domain/Industry Link
Configuration editing CLIs power real systems: CI/CD pipelines, trading engines’ risk limits, microservice feature flags, and infrastructure-as-code tooling. Skills here translate directly to building safe, automatable config workflows used by SREs, DevOps, and platform teams.

---

## Additional Resources

### Practical Resources
- Serde: *Quickstart*, *Derive*, *Data Model* sections.  
- `toml` & `serde_json` crate docs (value types, parsing/printing).  
- `clap` (v4) docs: subcommands, validators, `ArgAction`, and auto-help.  
- Blog posts on CLI UX patterns (help text, errors, exit codes).  

### Advanced Study
- Articles on **schema validation** patterns in Rust (custom validators, visitor patterns).  
- Property-based testing with `proptest` for round-trip invariants.  
- Papers/blogs on configuration drift and layered configuration systems.  

### Tool-Specific Resources
- **Serde:** common pitfalls (borrowed vs. owned, custom de/ser).  
- **toml/serde_json:** number handling, pretty printing, and pitfalls with null/None.  
- **clap:** best practices for error messages and discoverability (`--help`, `--version`, examples).  
- **Testing:** golden-test techniques and snapshot testing (e.g., `insta`) with care for determinism.  

---

## Implementation Notes
- Prefer **owned** config representations in the core to simplify lifetimes; convert at the edges if needed.  
- Represent paths as a sequence of **segments**: `Key("risk")`, `Key("max_order_size")`, `Index(1)`.  
- Separate **CLI parsing** (with `clap`) from **core operations** (`get/set/delete/list`) to keep logic testable.  
- Enforce **atomic writes**: write to `settings.toml.tmp` then `rename`.  
- Standardize errors with a small enum and implement `Display` to keep messages consistent.  

---

## Quick Consistency Check (Instructor Use)
- [x] Objectives use Bloom verbs and map to rubric  
- [x] Readings pinpoint specific chapters/sections  
- [x] Questions demand analysis/synthesis, not recall  
- [x] Exercises ramp up → project tasks  
- [x] Scope + commands are concrete and time-bounded  
- [x] Rubric totals 20 pts and aligns with objectives  
- [x] Excellence indicators are specific & measurable  
- [x] Resources are accessible/offline-friendly  
- [x] File paths/naming conventions are consistent  

