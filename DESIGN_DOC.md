# Config Parser CLI — One-Page Spec

## 1. Overview
CLI tool to read, edit, and write configuration files. Supports TOML and JSON. Provides a simple UX for managing settings in future projects.

---

## 2. Requirements
- **Commands**: read, set, delete, list  
- **Formats**: TOML + JSON  
- **Type preservation**:  
  - After load → mutate → save → reload, values keep original types (ints stay ints, bools stay bools, arrays/maps keep element types).  
  - **Value model**: Bool, String, Int/Uint/Float, Array, Object, Date/Time/DateTime (TOML), Null (JSON only).  
  - **Numbers**: keep Int vs Float distinct; no auto-promotion. CLI infers types (1500→Int, 1500.0→Float, true→Bool, "1500"→String). Type hints allowed (`path:int=1500`).  
  - **Dates/Times**: preserved in TOML; serialized as ISO-8601 strings in JSON.  
  - **Nulls**: supported in JSON; invalid in TOML (error instead of silent drop).  
  - **Arrays/Objects**: preserve order + types; out-of-bounds or missing parents error.  
  - **Type safety**: updates must match type unless `--coerce`.  
  - **Coercion (opt-in)**: allow String→Int/Float/Bool/Date, Int→Float, Date→String; disallow lossy conversions (e.g., Float→Int).  

---

## 3. Error Handling
- **Categories**: UsageError, FileIOError, ParseError, PathError, TypeError, ValidationError (stretch), NotSupported  
- **Exit codes**: `0` success · `2` UsageError · `3` FileIO/ParseError · `4` PathError · `5` TypeError/ValidationError · `6` NotSupported  
- **Message style**: one-line, actionable, always include category + path + expected vs actual + hint. Example:  
  `TypeError at network.timeout: expected Int, got String "1500". Try --coerce or --set network.timeout:int=1500.`  
- **Behavior**: fail fast · atomic writes (no partial files) · no silent coercions (`--coerce` required) · reject unsupported values (e.g., `null` in TOML)  
- **Examples**:  
  - FileIOError → `Could not read settings.toml: No such file or directory.`  
  - ParseError → `Invalid TOML at line 12, col 5: expected "]".`  
  - PathError → `servers[3].host: index 3 out of bounds (len=3).`  
  - NotSupported → `Key "x" is null, but TOML does not support null.`  
- **Testing**: unit tests per category · golden tests for failure cases · verify atomic write integrity  

---

## 4. Key-Path Grammar
- **Dot notation**: `network.timeout`, `database.host`  
- **Array indexing**: `servers[0].host` (0-based)  
- **Escaping**: use `\.` inside keys with dots, e.g. `metrics.requests\.per\.sec`  
- **Invalid paths**: missing parent, bad escape, out-of-bounds index → PathError  
- **Creation rules**: no auto-create of parents/arrays; must exist or error  

---

## 5. Data Model (IR)
- **Value enum**: Bool, String, Int, Uint, Float, Array, Object, Date/Time/DateTime (TOML), Null (JSON only)  
- **Ownership**: store owned values (no lifetimes) for simplicity  
- **Arrays**: preserve order + element types  
- **Objects**: string keys; no duplicates  
- **Constraints**: Int ≠ Float (no promotion), TOML cannot hold Null  

---

## 6. CLI UX
- **Subcommands**: `read`, `set`, `delete`, `list`  
- **Flags**: `--file <path>`, `--format <json|toml>`, `--out <path>`, `--coerce`, `--debug`  
- **Examples**:  
  - `config --file settings.toml read network.timeout`  
  - `config --file settings.toml set network.timeout=1500`  
  - `config --file settings.toml delete servers[1].host`  
- **Exit codes**: 0 success; non-zero per error category (see Error Handling)  
- **Help text**: concise descriptions + usage examples; discoverable with `--help`  
