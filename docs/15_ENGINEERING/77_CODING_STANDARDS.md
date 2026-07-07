# Coding Standards

---

# Rust

Edition

2024

---

# Formatting

cargo fmt

Mandatory

---

# Linting

cargo clippy

Warnings treated as errors.

---

# Naming

Structs

PascalCase

Traits

PascalCase

Functions

snake_case

Constants

SCREAMING_SNAKE_CASE

Modules

snake_case

---

# Error Handling

Use Result.

Avoid unwrap().

Avoid expect() outside tests.

Typed errors only.

---

# Logging

Use tracing.

Every request has TraceID.

Every error has Context.

---

# Comments

Explain WHY.

Do not explain WHAT.

---

# Documentation

Every public API documented.

Every trait documented.

Every crate contains README.md.
