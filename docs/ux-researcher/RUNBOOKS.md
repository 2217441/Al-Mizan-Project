# UX Researcher Runbooks

## Communication Protocol

### Research Context Assessment
Initialize UX research by understanding project needs.

**Research context query:**
```json
{
  "requesting_agent": "ux-researcher",
  "request_type": "get_research_context",
  "payload": {
    "query": "Research context needed: product stage, user segments, business goals, existing insights, design challenges, and success metrics."
  }
}
```

## Development Workflow

### 1. Research Planning
**Objective:** Understand objectives and design research approach.

**Planning priorities:**
- Define research questions
- Identify user segments
- Select methodologies
- Plan timeline
- Set success criteria

**Methodology selection:**
- Qualitative (Interviews, usability testing)
- Quantitative (Surveys, analytics)
- Mixed approaches

### 2. Implementation Phase
**Objective:** Conduct research and gather insights systematically.

**Steps:**
1. Recruit participants
2. Conduct sessions
3. Collect data
4. Analyze findings
5. Synthesize insights

**Progress Tracking JSON:**
```json
{
  "agent": "ux-researcher",
  "status": "analyzing",
  "progress": {
    "studies_completed": 0,
    "participants": 0,
    "insights_generated": 0,
    "design_impact": "pending"
  }
}
```

### 3. Impact Excellence
**Objective:** Ensure research drives meaningful improvements.

**Excellence Checklist:**
- [ ] Insights actionable
- [ ] Bias controlled
- [ ] Findings validated
- [ ] Recommendations clear
- [ ] Impact measured

## Research Methods

### User Interview Planning
- Research objectives
- Participant recruitment
- Interview guides
- Consent processes

### Usability Testing
- Test planning
- Task design
- Prototype preparation
- Data collection & analysis

### Survey Design
- Question formulation
- Response scales
- Logic branching
- Data analysis

### Analytics Interpretation
- Behavioral patterns
- Conversion funnels
- Drop-off analysis
- Heatmap insights
