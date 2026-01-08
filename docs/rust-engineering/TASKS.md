# Rust Engineering Tasks

**Last Updated**: 2026-01-07

---

## Active Sprint

### Completed ✅

- [x] Initial codebase assessment and workspace setup
- [x] Complete module-by-module code review
- [x] Create clippy configuration (`clippy.toml`)
- [x] Add `#![warn(clippy::pedantic)]` to crate root
- [x] Security hardening: JWT secret, SQL parameterization
- [x] Replace template handler unwraps
- [x] Add comprehensive domain tests (11 new tests)

### In Progress

- [/] Fix remaining clippy style warnings (27 low-priority)

### Deferred

- [ ] Add rustdoc to all public items
- [ ] Set up benchmark suite with Criterion
- [ ] Integration tests for API endpoints

---

## Test Coverage

| Module | Tests | Status |
|--------|-------|--------|
| `domain::authority` | 2 | ✅ |
| `domain::stability_tests::shariah` | 6 | ✅ |
| `domain::stability_tests::identity` | 1 | ✅ |
| `domain::stability_tests::model` | 4 | ✅ |
| `domain::tests` | 2 | ✅ |
| **Total** | **15** | ✅ |

---

## Backlog

### Performance

- [ ] Profile hot paths with `cargo flamegraph`
- [ ] Benchmark graph API response times
- [ ] Evaluate query parallelization opportunities

### Documentation

- [ ] Rustdoc for all public items
- [ ] Examples in documentation
- [ ] Architecture decision records

---

*Maintained by: Rust Engineer Agent*
