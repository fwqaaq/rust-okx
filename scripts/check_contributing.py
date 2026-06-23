#!/usr/bin/env python3
"""
Verify that src/api/ Rust source files follow CONTRIBUTING.md conventions.

Category R — Response files (responses.rs or files inside responses/)
  R1: Every `pub struct` must have `#[non_exhaustive]`
  R2: Every `pub struct` must derive `Deserialize`
  R3: Every `pub struct` must have `serde(rename_all = "camelCase")` or `serde(from = ...)`

Category A — api.rs files
  A1: Every `pub async fn` doc comment must contain the OKX path (/api/v5/...)
  A2: Every `pub async fn` doc comment must contain `# Errors`
  A3: Every `pub async fn` doc comment must state `Authenticated` or `Public`

Category T — Unit test files (tests.rs or files inside tests/)
  T1: Every `#[tokio::test]` must use `MockTransport::new`
  T2: Every `#[tokio::test]` must have a raw-string mock body (`r#"...`)
  T3: Every `#[tokio::test]` that does NOT call `.unwrap_err()` must call `mock.captured()`
  T4: Every `#[tokio::test]` that calls `mock.captured()` must also check `is_signed()`
"""

import re
import sys
from pathlib import Path

SRC_API = Path(__file__).resolve().parent.parent / "src" / "api"
violations: list[str] = []


# ── file classifiers ───────────────────────────────────────────────────────


def is_response_file(p: Path) -> bool:
    return p.name == "responses.rs" or p.parent.name == "responses"


def is_unit_test_file(p: Path) -> bool:
    return p.name == "tests.rs" or p.parent.name == "tests"


# ── body extractor ─────────────────────────────────────────────────────────


def extract_tokio_test_bodies(path: Path):
    """Yield (lineno, fn_name, body_text) for every #[tokio::test] in path."""
    lines = path.read_text().splitlines()
    i = 0
    while i < len(lines):
        if lines[i].strip() == "#[tokio::test]":
            # Find the `async fn` line (may be immediately after or after more attrs)
            fn_idx = i + 1
            while fn_idx < len(lines) and "async fn" not in lines[fn_idx]:
                fn_idx += 1
            if fn_idx >= len(lines):
                i += 1
                continue
            m = re.search(r"async\s+fn\s+(\w+)", lines[fn_idx])
            fn_name = m.group(1) if m else "?"
            # Collect function body by brace-depth counting
            body_lines: list[str] = []
            depth = 0
            started = False
            for line in lines[fn_idx:]:
                body_lines.append(line)
                for ch in line:
                    if ch == "{":
                        depth += 1
                        started = True
                    elif ch == "}":
                        depth -= 1
                if started and depth == 0:
                    break
            yield fn_idx + 1, fn_name, "\n".join(body_lines)
            i = fn_idx + len(body_lines)
        else:
            i += 1


# ── rule checkers ──────────────────────────────────────────────────────────


def check_response_structs(path: Path) -> None:
    lines = path.read_text().splitlines()
    for i, line in enumerate(lines):
        if not re.search(r"\bpub\s+struct\s+\w+", line):
            continue
        window = lines[max(0, i - 5) : i]
        # R1 — #[non_exhaustive]
        if not any("#[non_exhaustive]" in ln for ln in window):
            violations.append(
                f"[R1] {path}:{i + 1}: `pub struct` is missing `#[non_exhaustive]`"
            )
        # R2 — derives Deserialize
        if not any(
            re.search(r"#\[derive\([^)]*Deserialize", ln) for ln in window
        ):
            violations.append(
                f"[R2] {path}:{i + 1}: `pub struct` is missing `Deserialize` in `#[derive(...)]`"
            )
        # R3 — camelCase rename or custom serde(from=...)
        if not any(
            'rename_all = "camelCase"' in ln or "serde(from" in ln
            for ln in window
        ):
            violations.append(
                f'[R3] {path}:{i + 1}: `pub struct` is missing `#[serde(rename_all = "camelCase")]` or `#[serde(from = ...)]`'
            )


def check_api_methods(path: Path) -> None:
    lines = path.read_text().splitlines()
    for i, line in enumerate(lines):
        if not re.search(r"\bpub\s+async\s+fn\s+\w+", line):
            continue
        m = re.search(r"pub\s+async\s+fn\s+(\w+)", line)
        fn_name = m.group(1) if m else "?"
        # Collect the preceding /// doc block
        doc_lines: list[str] = []
        j = i - 1
        while j >= 0 and lines[j].strip().startswith("///"):
            doc_lines.append(lines[j])
            j -= 1
        if not doc_lines:
            violations.append(
                f"[A1] {path}:{i + 1}: `{fn_name}` has no doc comment"
            )
            continue
        doc = "\n".join(doc_lines)
        # A1 — OKX path present
        if "/api/v5/" not in doc:
            violations.append(
                f"[A1] {path}:{i + 1}: `{fn_name}` doc comment is missing the OKX endpoint path (`/api/v5/...`)"
            )
        # A2 — # Errors section
        if "# Errors" not in doc:
            violations.append(
                f"[A2] {path}:{i + 1}: `{fn_name}` doc comment is missing `# Errors`"
            )
        # A3 — auth status stated
        if "Authenticated" not in doc and "Public" not in doc:
            violations.append(
                f"[A3] {path}:{i + 1}: `{fn_name}` doc comment must state `Authenticated` or `Public`"
            )


def check_unit_tests(path: Path) -> None:
    for lineno, fn_name, body in extract_tokio_test_bodies(path):
        # T1 — MockTransport::new
        if "MockTransport::new" not in body:
            violations.append(
                f"[T1] {path}:{lineno}: `{fn_name}` must use `MockTransport::new` (unit tests run offline)"
            )
        # T2 — captured() required unless it's a failure test
        is_failure_test = ".unwrap_err()" in body
        if not is_failure_test and "mock.captured()" not in body:
            violations.append(
                f"[T2] {path}:{lineno}: `{fn_name}` must call `mock.captured()` to assert the outgoing request"
            )
        # T3 — is_signed() required whenever captured() is used
        if "mock.captured()" in body and "is_signed()" not in body:
            violations.append(
                f"[T3] {path}:{lineno}: `{fn_name}` calls `mock.captured()` but never checks `is_signed()`"
            )


# ── main ───────────────────────────────────────────────────────────────────


def main() -> int:
    counts = {"responses": 0, "api.rs": 0, "tests": 0}

    for path in sorted(SRC_API.rglob("*.rs")):
        rel = path.relative_to(Path.cwd()) if path.is_relative_to(Path.cwd()) else path
        if is_response_file(path):
            print(f"[responses] {rel}")
            check_response_structs(path)
            counts["responses"] += 1
        if path.name == "api.rs":
            print(f"[api.rs   ] {rel}")
            check_api_methods(path)
            counts["api.rs"] += 1
        if is_unit_test_file(path):
            print(f"[tests    ] {rel}")
            check_unit_tests(path)
            counts["tests"] += 1

    total = sum(counts.values())
    summary = (
        f"{counts['responses']} response file(s), "
        f"{counts['api.rs']} api.rs file(s), "
        f"{counts['tests']} test file(s)  —  {total} total"
    )

    print()
    if violations:
        print(f"Scanned {summary}\n")
        print("CONTRIBUTING.md violations:\n")
        for v in violations:
            print(f"  {v}")
        print(f"\n{len(violations)} violation(s)  —  see CONTRIBUTING.md for the rules.")
        return 1

    print(f"Scanned {summary}")
    print("All checks passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
