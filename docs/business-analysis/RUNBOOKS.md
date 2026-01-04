# Business Analysis Runbooks

> **Author**: Business Analyst Agent  
> **Date**: 2026-01-04  
> **Purpose**: Standard operating procedures for business analysis activities

---

## 1. Requirements Elicitation Runbook

### Purpose
Systematically gather and document requirements from stakeholders.

### Procedure

1. **Identify Stakeholders**
   - Review team.md and project documentation
   - Map influence/interest using quadrant chart
   - Prioritize engagement order

2. **Prepare Interview Template**
   ```markdown
   ## Stakeholder: [Name]
   ## Role: [Role]
   ## Date: [Date]
   
   ### Current Pain Points
   1. 
   
   ### Desired Outcomes
   1. 
   
   ### Success Criteria
   1. 
   
   ### Constraints/Dependencies
   1. 
   ```

3. **Conduct Discovery**
   - Schedule interview or async questionnaire
   - Follow prepared questions
   - Capture verbatim quotes

4. **Document Requirements**
   - Use standard format: `[ID] As a [role], I want [feature] so that [benefit]`
   - Assign priority (P0-P3)
   - Link to source stakeholder

5. **Validate & Approve**
   - Review with stakeholder
   - Get explicit sign-off
   - Update traceability matrix

### Output
- Updated requirements inventory in BUSINESS_ANALYSIS.md
- User stories in backlog format

---

## 2. Business Process Mapping Runbook

### Purpose
Visualize current and future state processes to identify optimization opportunities.

### Procedure

1. **Define Process Scope**
   - Name the process (e.g., "Data Ingestion", "Scholar Verification")
   - Identify start and end points
   - List participating actors

2. **Gather Process Information**
   - Review existing documentation
   - Trace code paths if needed
   - Interview process owners

3. **Create Process Diagram**
   - Use Mermaid flowchart syntax
   - Standard notation:
     - Rectangles: Activities
     - Diamonds: Decisions
     - Parallelograms: Inputs/Outputs
     - Swimlanes: Actors

4. **Identify Improvement Opportunities**
   - Bottlenecks (slow steps)
   - Waste (unnecessary steps)
   - Automation candidates
   - Error-prone areas

5. **Propose Future State**
   - Create "To-Be" diagram
   - Quantify expected improvements
   - Document assumptions

### Output
- Process diagrams in BUSINESS_ANALYSIS.md
- Improvement recommendations with ROI estimates

---

## 3. Risk Assessment Runbook

### Purpose
Identify, assess, and track risks to project success.

### Procedure

1. **Risk Identification**
   - Brainstorm with team
   - Review past project learnings
   - Analyze dependencies

2. **Risk Assessment Matrix**
   | Probability | Impact | Risk Level |
   |-------------|--------|------------|
   | High | High | 🔴 Critical |
   | High | Medium | 🟠 High |
   | Medium | High | 🟠 High |
   | Medium | Medium | 🟡 Medium |
   | Low | Any | 🟢 Low |

3. **Document Each Risk**
   ```markdown
   ### Risk: [Name]
   - **Category**: Strategic/Operational/Technical/Financial
   - **Probability**: High/Medium/Low
   - **Impact**: Critical/High/Medium/Low
   - **Description**: [Details]
   - **Mitigation**: [Actions]
   - **Owner**: [Person]
   - **Status**: Open/Mitigating/Closed
   ```

4. **Regular Review**
   - Weekly risk register review
   - Update probabilities based on new information
   - Close mitigated risks

### Output
- Risk register in BUSINESS_ANALYSIS.md
- Mitigation action items in TASKS.md

---

## 4. KPI Definition Runbook

### Purpose
Define measurable success indicators aligned with business objectives.

### Procedure

1. **Identify Business Objectives**
   - Extract from project vision
   - Align with stakeholder needs
   - Map to strategic goals

2. **Apply SMART Criteria**
   - **S**pecific: Clear definition
   - **M**easurable: Quantifiable
   - **A**chievable: Realistic target
   - **R**elevant: Aligned to objectives
   - **T**ime-bound: Deadline specified

3. **Define Measurement Method**
   | KPI | Data Source | Collection Frequency | Owner |
   |-----|-------------|---------------------|-------|
   | [Name] | [Source] | [Daily/Weekly/Monthly] | [Person] |

4. **Set Baselines & Targets**
   - Document current performance (baseline)
   - Define achievable target
   - Set stretch goal if applicable

5. **Create Dashboard Design**
   - Visual representation
   - Trend indicators
   - Alerting thresholds

### Output
- KPI framework in BUSINESS_ANALYSIS.md
- Dashboard specifications for DevOps

---

## 5. Stakeholder Communication Runbook

### Purpose
Maintain effective communication with all stakeholders.

### Procedure

1. **Segment Stakeholders**
   | Segment | Frequency | Channel | Content Type |
   |---------|-----------|---------|--------------|
   | Manage Closely | Weekly | Direct meeting | Detailed updates |
   | Keep Satisfied | Bi-weekly | Email summary | High-level status |
   | Keep Informed | Monthly | Newsletter | Progress highlights |
   | Monitor | Quarterly | Public updates | Major milestones |

2. **Prepare Communication**
   - Tailor message to audience
   - Lead with key decisions/blockers
   - Include clear CTAs

3. **Communication Template**
   ```markdown
   ## Status Update: [Date]
   
   ### 🟢 Wins This Period
   - 
   
   ### 🟡 In Progress
   - 
   
   ### 🔴 Blockers / Decisions Needed
   - 
   
   ### 📅 Next Period Focus
   - 
   ```

4. **Feedback Loop**
   - Request explicit feedback
   - Track sentiment over time
   - Adjust approach as needed

### Output
- Communication plan in project management artifacts
- Stakeholder satisfaction tracking

---

## 6. Change Request Runbook

### Purpose
Manage scope changes in a controlled manner.

### Procedure

1. **Receive Change Request**
   - Document requester and date
   - Capture detailed description
   - Assign tracking ID

2. **Impact Assessment**
   - Technical effort (with dev team)
   - Timeline impact
   - Cost implications
   - Risk considerations

3. **Prioritization**
   - Apply MoSCoW method
   - Compare against current backlog
   - Assess trade-offs

4. **Decision**
   - Approve → Add to backlog with priority
   - Defer → Add to future considerations
   - Reject → Document rationale

5. **Communication**
   - Notify requester of decision
   - Update relevant documentation
   - Communicate to team if approved

### Output
- Change request log
- Updated requirements/backlog

---

*This runbook is maintained by the Business Analyst Agent. Last updated: 2026-01-04T21:11:00+08:00*
