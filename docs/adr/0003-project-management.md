# 3. Agile Project Management & Git Flow

Date: 2025-12-17

## Status

Accepted

## Context

The project requires a structured yet flexible management methodology to handle the complexity of the Knowledge Graph implementation and ensure academic milestones are met.

## Decision

    *   **Increment**: Working software at the end of every sprint.

* **Roles**:
  * **Product Owner**: (User/Supervisor) Defines "what" to build.
  * **Development Team**: (Agent/User) Defines "how" to build.
  * **Scrum Master**: (Agent) Facilitates process.

### Design Philosophy: Domain-Driven Design (DDD)

* **Ubiquitous Language**: We will use strict terminology (e.g., *Naskh*, *Mansukh*, *Nasikh*) in both code and conversation.
* **Bounded Contexts**:
  * **Text Context**: Handling Quran/Hadith text integrity.
  * **Abrogation Context**: Handling the logic of abrogation relationships.
  * **Identity Context**: Handling users and scholars.

## Consequences

### Positive

* **Adaptability**: Agile allows us to pivot based on research findings.
* **Alignment**: DDD ensures the software model matches the mental model of Islamic scholars.
* **Rigor**: Defined processes prevent "spaghetti code" and scope creep.

### Negative

* **Overhead**: Requires maintaining backlog and documentation.
