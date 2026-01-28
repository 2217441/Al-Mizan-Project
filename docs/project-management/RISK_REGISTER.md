# Risk Register: Al-Mizan

> **Author**: Project Manager Agent
> **Date**: 2026-01-28
> **Status**: Active
> **SFIA Alignment**: PRMG Level 5 (Integrates robust risk management)

---

## Active Risks (Phase 1b)

| ID | Risk Description | Probability | Impact | Severity | Owner | Mitigation Strategy | Status |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **R-001** | **Integration Complexity**: Merging "Secular" SFIA data with "Tawhidic" Ontology may cause semantic incoherence. | Medium | High | **High** | Knowledge Engineer | Adopt "Ontological alignment" strategy mapping Skills to `Amal` subtypes. | 🟡 Monitoring |
| **R-002** | **Frontend Stagnation**: Lack of active frontend development ("Frontend Rescue") delays Stakeholder verification. | High | Medium | **High** | UI Designer | Prioritize "Verification Interface" wireframes in next sprint. | 🔴 Open |
| **R-003** | **Data Gaps**: Missing connection between imported Hadith nodes and Scholar narratives (Isnad connectivity). | High | High | **Critical** | Data Engineer | Request Sunnah.com API access; manually enrich Tier 1 nodes. | 🟡 Monitoring |
| **R-004** | **Scope Creep**: Adding "Agentic SFIA" features might distract from core "Quranic Graph" foundation. | Medium | Low | **Medium** | Product Manager | Strict backlog gating; mark SFIA as "Agent Capability" only, no UI. | 🟢 Mitigated |

## Closed Risks

| ID | Description | Resolution | Date |
| :--- | :--- | :--- | :--- |
| **R-000** | **Archive Confusion**: Old documentation cluttering workspace. | Archived to `docs/archive` and `~/.gemini/archive`. | 2026-01-28 |
