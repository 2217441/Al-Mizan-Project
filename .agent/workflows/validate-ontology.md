---
description: Validates the Knowledge Graph ontology and SurrealDB schema rules.
---

# Al-Mizan: Validate Ontology

Use this workflow to ensure the Knowledge Graph adheres to the established ontology.

## Instructions
1.  **Check Schema**: Run `surreal sql` to verify table and field definitions.
2.  **Verify Nodes/Edges**: Check that mandatory relations (e.g., `BELONGS_TO`, `PART_OF`) are correctly linked.
3.  **Run Validation Scripts**: If there are Python or JS validation scripts, execute them.
4.  **Report Gaps**: Document any nodes that violate the ontology rules.
