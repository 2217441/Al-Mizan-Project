# Stakeholder Communication Plan

> **Project**: Al-Mizan Tawhidic Knowledge Graph  
> **Author**: Business Analyst Agent  
> **Date**: 2026-01-04  
> **Status**: Active  
> **Purpose**: Coordinate messaging and engagement for FYP evaluation and beyond

---

## 1. Executive Summary

This plan outlines the communication strategy for engaging all Al-Mizan stakeholders during the critical FYP evaluation phase (January 2026) and establishes foundations for post-FYP growth. The goal is to ensure evaluators understand the project's technical depth and strategic vision, while keeping the development team aligned.

---

## 2. Stakeholder Register

### 2.1 Primary Stakeholders (Manage Closely)

| Stakeholder | Role | Interest | Influence | Key Concerns | Communication Owner |
|-------------|------|----------|-----------|--------------|---------------------|
| **Prof. Sharyar Wani** | FYP Supervisor | High | High | Academic rigor, timeline adherence, novelty | Project Lead |
| **FYP Evaluation Panel** | Evaluators | High | Critical | Technical depth, demo quality, documentation | Project Lead + All Agents |
| **Ammar Qasiem** | Lead Engineer | High | High | Architecture integrity, code quality | Self |
| **Muhammad Firdaus** | Data Scientist | High | High | ETL accuracy, AI integration | Self |

### 2.2 Secondary Stakeholders (Keep Satisfied)

| Stakeholder | Role | Interest | Influence | Key Concerns | Communication Owner |
|-------------|------|----------|-----------|--------------|---------------------|
| **IIUM KICT Faculty** | Academic Institution | Medium | Medium | Compliance, ethics, Islamic values | Project Lead |
| **Potential Research Collaborators** | Future Partners | Medium | Low | API access, data quality | Business Analyst |

### 2.3 Tertiary Stakeholders (Keep Informed)

| Stakeholder | Role | Interest | Influence | Key Concerns | Communication Owner |
|-------------|------|----------|-----------|--------------|---------------------|
| **Islamic Scholars (Future)** | Verification Users | Low (now) | High (later) | Usability, theological accuracy | Product Manager |
| **EdTech/FinTech Companies** | B2B Customers | Low (now) | Medium (later) | API reliability, pricing | Business Analyst |
| **Open Source Community** | Contributors | Low | Low | Documentation, contribution guide | Technical Writer |

---

## 3. Communication Matrix

### 3.1 FYP Evaluation Phase (Current Priority)

| Audience | Message Theme | Channel | Frequency | Owner | Deliverable |
|----------|---------------|---------|-----------|-------|-------------|
| **Supervisor** | Progress updates, blockers | Email + Meeting | Weekly | Project Lead | Status report |
| **Evaluation Panel** | Technical showcase | Presentation | One-time (Eval Day) | All | Demo + Report |
| **Dev Team** | Sprint sync, priorities | Standup | Daily | Project Manager | Task board updates |

### 3.2 Post-FYP Phase (Foundation for Growth)

| Audience | Message Theme | Channel | Frequency | Owner | Deliverable |
|----------|---------------|---------|-----------|-------|-------------|
| **Potential Partners** | Value proposition | LinkedIn + Email | Monthly | Business Analyst | Pitch deck |
| **Research Community** | Technical findings | Blog / Paper | Quarterly | Technical Writer | Publication |
| **Open Source Users** | Getting started | GitHub + Docs | On-demand | Technical Writer | README + Guides |

---

## 4. FYP Evaluation Messaging Strategy

### 4.1 Core Narrative

> **Elevator Pitch** (30 seconds):
> "Al-Mizan is a Tawhidic Knowledge Graph that solves the 'epistemological fragmentation' problem in Islamic digital resources. Unlike keyword-based search engines, our graph-based system preserves the sacred chain of transmission (Isnad), enabling AI agents to query verified theological truth rather than hallucinate answers."

### 4.2 Key Messages by Audience

| Evaluator Type | Key Message | Supporting Evidence |
|----------------|-------------|---------------------|
| **Technical Evaluator** | "Sub-10ms graph traversal with type-safe Rust backend" | Performance benchmarks, schema design |
| **Research Evaluator** | "First Tawhidic ontology enforcing Isnad computationally" | Literature review, FYP report |
| **Industry Evaluator** | "B2B API ready for Islamic Finance compliance" | Enterprise module demo, certification endpoint |

### 4.3 Demo Script Outline

```
1. HOOK (30s)
   - "What if an AI gave you a wrong fatwa?"
   - Show problem: ChatGPT hallucinating Islamic ruling

2. SOLUTION INTRO (1m)
   - Introduce Al-Mizan as "Ground Truth API"
   - Show architecture diagram

3. LIVE DEMO (5m)
   - Query a verse → Show connected Hadith → Show derived ruling
   - Demonstrate graph traversal in real-time
   - Show enterprise compliance check (Murabaha contract)

4. TECHNICAL DEPTH (3m)
   - Explain Fitna Defense Protocol
   - Show Algorithmic Shura (reputation-weighted consensus)
   - Highlight Rust type safety + SurrealDB choice

5. FUTURE VISION (1m)
   - Phase 2: Cyborg Isnad (AI + Human verification)
   - Phase 3: B2B API monetization

6. Q&A
```

---

## 5. Communication Templates

### 5.1 Weekly Status Update (To Supervisor)

```markdown
## Al-Mizan Weekly Update | [DATE]

### 🟢 Completed This Week
- [Item 1]
- [Item 2]

### 🟡 In Progress
- [Item with % completion]

### 🔴 Blockers / Risks
- [Blocker and proposed mitigation]

### 📅 Next Week Focus
- [Priority 1]
- [Priority 2]

### ❓ Questions / Decisions Needed
- [Question for supervisor]
```

### 5.2 Team Announcement (Internal)

```markdown
## 📢 Team Update | [DATE]

**Priority Shift**: [Reason]

### Action Items
- @[Agent]: [Task] by [Date]
- @[Agent]: [Task] by [Date]

### Context
[Brief explanation of why this matters]
```

### 5.3 External Inquiry Response (Future Partners)

```markdown
Subject: Re: Al-Mizan API Inquiry

Dear [Name],

Thank you for your interest in Al-Mizan.

**Current Status**: We are completing our academic validation phase (FYP) and will release a public API in [Timeline].

**What We Offer**:
- Graph-based Islamic knowledge queries
- Shariah compliance verification for financial contracts
- Isnad-verified rulings with full provenance

**Next Steps**: I'd be happy to schedule a demo call. Please let me know your availability.

Best regards,
[Name]
```

---

## 6. Escalation Protocol

### 6.1 Issue Severity Levels

| Level | Definition | Response Time | Escalation Path |
|-------|------------|---------------|-----------------|
| **P0 - Critical** | Demo broken, data corruption | Immediate | All hands on deck |
| **P1 - High** | Major feature regression | < 4 hours | Project Lead + relevant agent |
| **P2 - Medium** | Non-blocking issue | < 24 hours | Assigned agent |
| **P3 - Low** | Enhancement request | Next sprint | Backlog |

### 6.2 Escalation Contacts

| Role | Primary Contact | Backup |
|------|-----------------|--------|
| **Technical Issues** | Ammar (Lead Engineer) | DevOps Agent |
| **Data Issues** | Firdaus (Data Scientist) | ETL scripts |
| **Documentation** | Technical Writer Agent | Business Analyst |
| **Stakeholder Concerns** | Business Analyst | Project Manager |

---

## 7. Feedback Loops

### 7.1 Internal Feedback

| Source | Collection Method | Frequency | Action Owner |
|--------|-------------------|-----------|--------------|
| Agent team | Async retrospective | Weekly | Project Manager |
| Code reviews | PR comments | Continuous | Dev team |
| Task blockers | TASKS.md updates | Real-time | All agents |

### 7.2 External Feedback (Post-FYP)

| Source | Collection Method | Frequency | Action Owner |
|--------|-------------------|-----------|--------------|
| Beta users | Survey / Interview | Monthly | UX Researcher |
| API consumers | Usage analytics | Weekly | Product Manager |
| Scholars | Verification feedback | Per session | Product Manager |

---

## 8. Calendar of Key Dates

| Date | Event | Stakeholders | Preparation Required |
|------|-------|--------------|---------------------|
| **2026-01-XX** | FYP 1 Evaluation | Panel, Supervisor | Demo, Report, Presentation |
| **2026-02-XX** | FYP 2 Kickoff | Team | Phase 2 planning |
| **2026-06-XX** | FYP 2 Final Evaluation | Panel | Full system demo |
| **2026-Q3** | Public API Launch | Partners, Community | Documentation, Pricing |

> **Note**: Specific dates to be confirmed with Project Manager and Supervisor.

---

## 9. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Supervisor satisfaction** | Positive feedback | Verbal/written confirmation |
| **Evaluation score** | ≥ 85% | Official grading |
| **Demo completion** | 100% features shown | Checklist |
| **Q&A confidence** | All questions answered | Panel feedback |
| **Documentation completeness** | 80%+ | Coverage audit |

---

## 10. Document Control

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-04 | Business Analyst Agent | Initial creation |

---

*This document is maintained by the Business Analyst Agent. Coordinate with Project Manager for updates.*
