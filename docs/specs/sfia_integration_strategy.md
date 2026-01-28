# SFIA Integration Strategy: Agent-Centric Knowledge Base

**Purpose**: Define the integration of the SFIA 9 Framework into the Al-Mizan Project, providing AI Agents with a standardized vocabulary for **Professional Capabilities**.

## 1. Context & Motivation
The project requires a structured vocabulary to describe professional actions, skills, and responsibilities. SFIA (Skills Framework for the Information Age) provides this global standard.

Integrating SFIA allows Agents to:
1.  **Classify Work**: Map human activities (coding, managing, designing) to standardized skills.
2.  **Assess Capability**: Evaluate if an entity (user or agent) has the capacity to perform a task.
3.  **Recommend Growth**: Suggest development paths aligned with professional standards.

## 2. Terminology Alignment (Standardized English)
We will map SFIA concepts to the project's core universal ontology using standard English terminology to ensure broad public accessibility.

| SFIA Concept | Core Ontology Concept | Relation |
| :--- | :--- | :--- |
| **Skill** (e.g., Programming) | **Action / Work** | `is_type_of` |
| **Level** (1-7) | **Rank / Degree** | `has_level` |
| **Responsibility** | **Accountability** | `implies` |
| **Professionalism** | **Excellence** | `manifests_as` |
| **Code of Ethics** | **Conduct** | `aligned_with` |

## 3. Architecture: Markdown & JSON First (Lightweight)
To reduce complexity, we will strictly utilize the **File-Based Artifacts** (JSON/Markdown) for the Agent's "Reference Memory".

*   **Read-Only Reference**: Agents will read the `excel_skills.json` or formatted Markdown files directly (via Context or RAG) to understand skill definitions.
*   **No Immediate Database Schema**: We will **skip** creating a SurrealDB schema for the static SFIA data. Agents do not need a database to read definitions; they only need the text.
*   **Future Use Case**: A database may be introduced later *only* if we need to track dynamic User Progress (state), but the SFIA dictionary itself stays in files.

## 4. Agent Tooling (Retrieval Based)
Agents will use simple file reading or search tools to access this data.

1.  **`read_skill_definition(skill_code)`**: Reads the specific JSON/Markdown entry.
2.  **`search_skills(keyword)`**: Greps the `all_skills.json` or Markdown folder.
