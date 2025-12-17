# CSCI 4401/4402 FINAL YEAR PROJECT PROPOSAL

**PROJECT TITLE**: Al-Mizan: A Neuro-Symbolic Knowledge Graph Framework for Islamic Knowledge Representation

**(For Group Project)**

**MEMBER 1 (Software Engineering Track):**

* **NAME**: Ammar Qasiem Footen bin John Anthony Footen
* **MATRIC NO**: 2217441
* **EMAIL**: <ammarqasiem01@gmail.com>

**MEMBER 2 (Data Science Track):**

* **NAME**: Firdaus [Insert Full Name]
* **MATRIC NO**: [Insert Matric No]
* **EMAIL**: [Insert Gmail]

---

## 1.1 INTRODUCTION

This project involves the design and development of "**Al-Mizan**," a specialized Graph-based API and Data Visualization Application tailored for Islamic Knowledge Representation. While current digital Islamic resources predominantly rely on Relational Database Management Systems (RDBMS) and keyword-based indexing, these systems are fundamentally limited in capturing the intricate, multi-dimensional connections inherent in Islamic scholarship (e.g., an Isnad chain, a Fiqh ruling derived from a Hadith, or an Abrogation relationship between verses).

The core specification of this application is the transition from Linear Data Storage to a **Neuro-Symbolic Knowledge Graph**. Using **SurrealDB** and **Rust**, the system will model data not as isolated rows in tables, but as Nodes (Entities) and Edges (Relationships). The final deliverable will consist of a high-performance backend API (Member 1) populated by an AI-assisted data pipeline (Member 2), visualized via a web dashboard.

### 1.1.1 Research Hypothesis

To fulfill the research component of this study, the project posits the following hypothesis regarding data modeling efficiency:

* **Null Hypothesis ($H_0$)**: There is no significant difference in query complexity or traversal latency between Relational (SQL) and Graph-based models when representing recursive Islamic genealogical structures (Isnad).
* **Alternative Hypothesis ($H_1$)**: The Graph-based model will demonstrate a statistically significant reduction in query complexity (approaching $O(1)$ adjacency lookups vs $O(n^2)$ SQL joins) and reduced latency for recursive depth traversals > 3 levels.

## 1.2 PROBLEM DESCRIPTION

### 1.2.1 Background of the Problem (The Epistemological Gap)

The digital Islamic ecosystem currently suffers from an "**Epistemological Incongruence**." While Islamic knowledge is inherently relational—relying on complex chains of authority (Sanad), inter-textual references (Tafsir), and derivation logic (Istinbat)—the underlying digital infrastructure hosting this knowledge is built on secular, linear paradigms [1].

Currently, Islamic applications function as isolated "Data Silos." A user reading a digital Quran cannot instantly traverse the graph to see which Hadith explains a specific verse. This forces researchers to manually reconstruct these chains. The problem is not a lack of digitization, but a lack of *structural connectivity* [2].

### 1.2.2 Problem Statement

1. **Structural Mismatch (The Research Problem)**: Traditional RDBMS utilize rigid table structures that fail to natively represent the recursive nature of Isnad. Modeling these in SQL leads to excessive JOIN operations and performance degradation.
2. **Semantic Discontinuity**: Existing APIs provide data as static text strings without metadata regarding their reliability. There is no machine-readable standard to indicate that "Ruling A" is abrogated by "Ruling B."
3. **Fragmentation of Evidence**: There is no unified, verified protocol that links Primary Sources (Quran/Sunnah) to Secondary Derivations (Fiqh/Fatwa) in a single queryable environment.

## 1.3 OBJECTIVES

1. **To Formulate (Research - Shared)**: A Domain-Specific Ontology (DSO) that formalizes the "Stratification of Truth" in Islamic epistemology, distinguishing between Immutable Constants (Al-Thawabit) and Interpretive Variables (Al-Ijtihadiyyat).
2. **To Engineer (Development - Member 1)**: A high-performance Graph API using Rust and SurrealDB that implements this ontology, enforcing logical constraints and exposing secure endpoints.
3. **To Implement (Data Science - Member 2)**: A Neuro-Symbolic Pipeline using Python/NLP that leverages Large Language Models (LLMs) to draft relationships from unstructured text, which are then grounded into the strict Symbolic Graph logic.
4. **To Benchmark (Analysis - Member 2)**: To conduct a quantitative performance analysis comparing the proposed Graph solution against a standard SQL baseline, measuring Query Latency (ms) and statistical significance of the $H_1$ hypothesis.

## 1.4 PROJECT SCOPE

### 1.4.1 Users

* **Primary Users**: Islamic Software Engineers requiring a "Truth-Layer" infrastructure.
* **Secondary Users**: Digital Humanities Researchers (e.g., IRKHS Postgraduates) analyzing citation networks.
* **Tertiary Users**: Educational Content Creators (EdTech) requiring curriculum dependency maps.

### 1.4.2 Target Market / Sample (Data Methodology)

To balance scalability with verification, the project adopts a Tiered Data Strategy:

* **Tier 1 (Global Ingestion)**: The system will ingest the Full Corpus of the Quran and Kutub Sittah as "Text Nodes" via Python scripts.
* **Tier 2 (Semantic Graph)**: Verified Edges will be limited to Juz Amma, 40 Hadith, and the Chapter of Purification.

### 1.4.3 Specific Platform & Work Division

The project employs a Service-Oriented Architecture to split responsibilities:

**Member 1 (System Architect - Ammar):**

* **Core Engine**: SurrealDB (Graph Database) configured with Unicode/UTF-8 support to handle mixed-script storage (Arabic uthmani and English).
* **Backend**: Rust (Axum) for the high-performance API Layer.
* **Infrastructure**: Docker for containerization and security (OpSec).
* **Frontend**: D3.js with RTL (Right-to-Left) support for proper visualization of Arabic nodes alongside English navigation.
* **Management Protocol**: Agile (Scrum) with weekly Sprints to manage the API lifecycle.

**Member 2 (Data Scientist - Firdaus):**

* **Data Pipeline**: Python (Pandas/BeautifulSoup) for scraping and cleaning data from Tanzil.net.
* **Neuro-Symbolic Processing**: Graph RAG (Retrieval Augmented Generation) workflow using LangChain to extract entities from Tafsir texts.
* **Analysis**: Jupyter Notebooks for running the SQL vs. Graph benchmarks and generating statistical charts.
* **Research Protocol**: DSRM (Design Science Research Methodology) for hypothesis validation.

### 1.4.4 Features and Functionalities

* **Semantic Traversal Engine**: A query interface allowing deep navigation (e.g., Query: Show me the graph path from Verse 5:6 to the Shafi'i ruling on Wudu).
* **Multilingual Resolution**: A "Concept-Level" node architecture that maps queries in English (e.g., "Charity") to the correct Theological Nodes in Arabic (e.g., "Zakat" or "Sadaqah"), enabling non-Arabic speakers to query deep sources.
* **Epistemological Tiering System**: A metadata logic that tags every data node as either Thabit (Immutable) or Zanni (Probabilistic).
* **Visual Isnad Rendering**: An automated graphical view that draws the tree of narrators for any requested Hadith.

## 1.5 CONSTRAINTS

* **Operational Security (OpSec)**: The prototype will operate on a Read-Only model for public interfaces to prevent data injection.
* **AI Hallucination Risk**: AI (Member 2's pipeline) is strictly used for "Drafting." All logic commits to the database require manual verification.
* **Madhab Complexity**: The system focuses on a single dominant opinion per issue for architectural validation.

## 1.6 PROJECT STAGES (Gantt Chart Overview)

* **Weeks 1-3**:
  * Shared: Literature Review & Ontology Definition.
  * Member 2: Data collection (Python scraping) of Raw Text.
* **Weeks 4-5**:
  * Member 1: Schema Engineering (SurrealDB) & API Setup (Rust).
  * Member 2: NLP Pipeline Setup for Edge Extraction.
* **Weeks 6-10**:
  * Member 1: API Implementation, Auth Logic, and Docker Environment.
  * Member 2: Data Ingestion, Cleaning, and Manual Verification of Tier 2 edges.
* **Weeks 11-12**:
  * Member 1: Frontend Integration (Visualization).
  * Member 2: Benchmarking: Running the latency tests ($N=1000$) and producing the Statistical Analysis Report.
* **Weeks 13-14**: Final Report integration and Innovatex preparation.

## 1.7 SIGNIFICANCE OF THE PROJECT

* **Scientific Contribution**: This project contributes to the field of **Neuro-Symbolic AI**, demonstrating how Graph Theory can ground Large Language Models (LLMs) in non-Western epistemologies.
* **Research Acceleration**: The system empowers scholars (e.g., IRKHS) by automating citation verification. Navigating Isnad chains and Tafsir links, which traditionally takes hours, is reduced to milliseconds.
* **Educational Utility (EdTech)**: The graph's "Dependency Logic" (e.g., Wudu is a condition for Salah) functions as an automated Curriculum Generator, enabling EdTech platforms to dynamically structure learning paths based on logical prerequisites.
* **Ecosystem Infrastructure**: It serves as a foundational protocol for the broader Islamic Digital Economy. By providing a verified API for "Halal Logic," it enables downstream innovations in Halal Supply Chain and Automated Compliance (RegTech).

## 1.8 SUMMARY

The Al-Mizan project is a research-driven initiative to re-architect the digital foundation of Islamic knowledge. By splitting the workflow into Systems Engineering (Rust/Graph) and Data Science (Python/Neuro-Symbolic AI), the team aims to prove that semantic connectivity is the key to unlocking the next generation of Islamic digital applications, validated through rigorous statistical benchmarking.

## 1.9 REFERENCES

[1] T. Berners-Lee, J. Hendler, and O. Lassila, "The Semantic Web," Scientific American, vol. 284, no. 5, pp. 34–43, 2001.

[2] A. A. Azman, et al., "The Challenges of Ontology Development for Islamic Knowledge," International Journal of Advanced Computer Science and Applications, vol. 11, no. 5, 2020.

[3] I. Robinson, J. Webber, and E. Eifrem, Graph Databases: New Opportunities for Connected Data. O'Reilly Media, Inc., 2015.
