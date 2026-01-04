# Technical Writing Runbooks

> **Author**: Technical Writing Agent  
> **Date**: 2026-01-04  
> **Purpose**: Standard procedures for documentation tasks

---

## 1. Creating New Documentation

### Pre-Flight Checklist

Before creating new documentation:

- [ ] Identify target audience
- [ ] Check for existing related docs
- [ ] Gather source material (code, specs, team input)
- [ ] Choose appropriate format (guide, reference, tutorial)
- [ ] Plan information architecture

### Standard Document Structure

```markdown
# Document Title

> **Author**: [Agent Name]  
> **Date**: YYYY-MM-DD  
> **Status**: Draft | Review | Published

---

## Overview

Brief introduction to the topic.

---

## Section 1

Content with appropriate subsections.

---

## Summary

Key takeaways.

---

*Last updated: YYYY-MM-DDTHH:MM:SS+TZ*
```

### Post-Creation

- [ ] Add to navigation/index
- [ ] Cross-reference related docs
- [ ] Verify all links work
- [ ] Check code examples run correctly
- [ ] Request peer review if needed

---

## 2. API Documentation

### Endpoint Documentation Template

```markdown
## `METHOD /api/v1/endpoint`

Brief description of what this endpoint does.

### Request

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| Content-Type | application/json | Yes |
| Authorization | Bearer {token} | Yes |

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| id | string | Yes | Resource identifier |

**Body**:
```json
{
  "field": "value"
}
```

### Response

**Success (200)**:
```json
{
  "data": {...}
}
```

**Error (4xx/5xx)**:
```json
{
  "error": "Error message"
}
```

### Example

```bash
curl -X GET https://api.example.com/v1/endpoint \
  -H "Authorization: Bearer token"
```
```

### API Documentation Checklist

- [ ] All endpoints documented
- [ ] Request/response examples provided
- [ ] Error codes explained
- [ ] Authentication described
- [ ] Rate limits documented
- [ ] Versioning policy stated

---

## 3. Schema Documentation

### Table Documentation Template

```markdown
## `table_name`

Description of what this table stores.

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | record | Yes | Primary identifier |
| name | string | Yes | Human-readable name |
| created_at | datetime | Yes | Creation timestamp |

### Relationships

| Direction | Target | Type | Description |
|-----------|--------|------|-------------|
| -> | other_table | relates_to | Describes relationship |

### Indexes

| Index | Fields | Purpose |
|-------|--------|---------|
| idx_name | field1, field2 | Query optimization |

### Example

```surql
SELECT * FROM table_name WHERE field = 'value';
```
```

---

## 4. Tutorial Writing

### Tutorial Structure

1. **Introduction** - What will be learned
2. **Prerequisites** - Required knowledge/setup
3. **Steps** - Numbered, clear instructions
4. **Verification** - How to confirm success
5. **Next Steps** - Where to go from here

### Writing Guidelines

- Use active voice ("Click the button" not "The button should be clicked")
- Include screenshots for UI actions
- Provide copy-pastable code examples
- Explain the "why" not just the "what"
- Test all steps before publishing

---

## 5. Review Process

### Self-Review Checklist

- [ ] Spelling and grammar checked
- [ ] Technical accuracy verified
- [ ] Examples tested and working
- [ ] Links validated
- [ ] Consistent formatting
- [ ] Appropriate for audience

### Peer Review Request

When requesting review, include:

1. Document link
2. Target audience
3. Specific areas needing feedback
4. Deadline for review

### Publishing

- [ ] Final review complete
- [ ] Version number updated
- [ ] Changelog entry added
- [ ] Navigation updated
- [ ] Announced to stakeholders

---

## 6. Maintenance

### Regular Tasks

| Task | Frequency | Description |
|------|-----------|-------------|
| Link audit | Monthly | Check for broken links |
| Freshness review | Quarterly | Identify stale content |
| User feedback | Ongoing | Address reported issues |
| Code example testing | On release | Verify examples still work |

### Deprecation Process

1. Mark document as deprecated with banner
2. Add redirect to replacement (if applicable)
3. Keep available for 2 major versions
4. Archive (don't delete) when removing

---

## 7. Style Quick Reference

### Voice and Tone

- Professional but approachable
- Assume intelligence, not knowledge
- Be concise but complete
- Use "you" to address the reader

### Formatting Standards

| Element | Format |
|---------|--------|
| File names | `backticks` |
| UI elements | **bold** |
| First mention of term | *italics* |
| Code | `backticks` or fenced blocks |
| Commands | Code blocks with language |

### Status Indicators

| Emoji | Meaning |
|-------|---------|
| 🔴 | Missing/Critical |
| 🟡 | Partial/In Progress |
| 🟢 | Complete/Good |
| ⏳ | In Progress |
| ✅ | Completed |

---

*Last updated: 2026-01-04T20:51:43+08:00*
