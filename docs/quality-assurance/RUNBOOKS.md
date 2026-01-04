# Quality Assurance Runbooks

> **Author**: QA Expert Agent  
> **Date**: 2026-01-04  
> **Purpose**: Standard operating procedures for QA activities in Al-Mizan

---

## 1. API Test Case Design Runbook

### Purpose
Create comprehensive test cases for REST API endpoints.

### Procedure

1. **Endpoint Documentation Review**
   - Check OpenAPI/Swagger spec
   - Review API_REFERENCE.md in technical-writing
   - Note request/response schemas

2. **Test Case Categories**
   | Category | Description | Example |
   |----------|-------------|---------|
   | **Happy Path** | Valid input, expected output | GET /quran/1/1 returns Al-Fatiha |
   | **Boundary** | Edge values | Query limit=0, limit=1000 |
   | **Negative** | Invalid input handling | GET /quran/999/999 returns 404 |
   | **Security** | Auth, injection, access | Invalid API key rejected |
   | **Performance** | Response time validation | < 50ms p99 |

3. **Test Case Template**
   ```markdown
   ## TC-[ID]: [Test Case Name]
   
   **Endpoint**: [Method] [Path]
   **Category**: [Happy Path / Boundary / Negative / Security / Performance]
   **Priority**: [P0 / P1 / P2]
   
   ### Preconditions
   - [Required state or data]
   
   ### Test Data
   - Request: [Body / Params]
   
   ### Expected Result
   - Status Code: [200 / 404 / etc.]
   - Response: [Expected body content]
   
   ### Actual Result
   - [Filled during execution]
   
   ### Status
   - [ ] Pass / [ ] Fail / [ ] Blocked
   ```

4. **Coverage Matrix**
   | Endpoint | Happy | Boundary | Negative | Security | Perf |
   |----------|-------|----------|----------|----------|------|
   | GET /quran/{surah}/{ayah} | ✅ | ✅ | ✅ | ✅ | ✅ |
   | GET /hadith/{collection}/{number} | | | | | |

### Output
- Test cases in `/test/api/` directory
- Coverage matrix in QA_ANALYSIS.md

---

## 2. Defect Reporting Runbook

### Purpose
Standardize defect documentation for efficient resolution.

### Procedure

1. **Severity Classification**
   | Severity | Definition | Example | SLA |
   |----------|------------|---------|-----|
   | 🔴 **Critical** | System down, data corruption | Database connection lost | 4 hrs |
   | 🟠 **High** | Major feature broken | API returns wrong data | 1 day |
   | 🟡 **Medium** | Feature degraded | Slow response times | 1 week |
   | 🟢 **Low** | Cosmetic, minor | Typo in error message | 2 weeks |

2. **Bug Report Template**
   ```markdown
   ## BUG-[ID]: [Brief Title]
   
   **Severity**: [Critical / High / Medium / Low]
   **Priority**: [P0 / P1 / P2 / P3]
   **Component**: [API / Frontend / Database / Infrastructure]
   **Reporter**: QA Expert Agent
   **Date Found**: [Date]
   
   ### Environment
   - Branch/Commit: [hash]
   - OS: [platform]
   - SurrealDB version: [version]
   
   ### Steps to Reproduce
   1. [Step 1]
   2. [Step 2]
   3. [Step 3]
   
   ### Expected Behavior
   [What should happen]
   
   ### Actual Behavior
   [What actually happens]
   
   ### Evidence
   - Logs: [link or snippet]
   - Screenshot: [if applicable]
   - API Response: [if applicable]
   
   ### Root Cause (if known)
   [Analysis]
   
   ### Suggested Fix (if known)
   [Recommendation]
   ```

3. **Triage Process**
   - New bugs reviewed within 24 hours
   - Assign ownership
   - Validate severity
   - Add to sprint if P0/P1

### Output
- GitHub Issues with bug label
- Defect metrics in QA dashboard

---

## 3. Regression Testing Runbook

### Purpose
Ensure new changes don't break existing functionality.

### Procedure

1. **Trigger Conditions**
   | Event | Regression Scope |
   |-------|------------------|
   | Feature branch merge | Modified components |
   | Release candidate | Full regression |
   | Hotfix | Affected area + smoke |
   | Dependency update | Integration tests |

2. **Test Selection**
   ```
   Priority Order:
   1. P0 Critical path tests (always run)
   2. P1 Component-specific tests (affected areas)
   3. P2 Extended coverage (time permitting)
   ```

3. **Execution Checklist**
   - [ ] Pull latest from target branch
   - [ ] Reset test database to clean state
   - [ ] Run automated suite
   - [ ] Execute manual P0 cases
   - [ ] Document any new failures
   - [ ] Compare results with baseline

4. **Pass/Fail Criteria**
   | Criterion | Threshold |
   |-----------|-----------|
   | P0 tests | 100% pass |
   | P1 tests | > 95% pass |
   | P2 tests | > 90% pass |
   | New failures | 0 (investigate all) |
   | Performance | Within baseline ± 10% |

5. **Reporting**
   ```markdown
   ## Regression Report: [Date]
   
   **Branch**: [branch]
   **Commit**: [hash]
   
   ### Summary
   | Level | Total | Pass | Fail | Skip |
   |-------|-------|------|------|------|
   | P0 | X | X | 0 | 0 |
   | P1 | X | X | X | X |
   | P2 | X | X | X | X |
   
   ### Failures
   - [List any failures with links to bugs]
   
   ### Blockers
   - [Any blocking issues]
   
   ### Recommendation
   - [ ] ✅ Proceed to deploy
   - [ ] ⚠️ Proceed with known issues
   - [ ] 🛑 Block deployment
   ```

### Output
- Regression report in CI artifacts
- Go/No-go recommendation

---

## 4. Data Integrity Validation Runbook

### Purpose
Verify correctness and completeness of Tawhidic Knowledge Graph data.

### Procedure

1. **Quran Data Validation**
   | Check | Query | Expected |
   |-------|-------|----------|
   | Total verses | `SELECT count() FROM ayah` | 6,236 |
   | Surah count | `SELECT count() FROM surah` | 114 |
   | Verse integrity | Check for null Arabic text | 0 nulls |
   | Unicode validity | RTL marker presence | All pass |

2. **Hadith Data Validation**
   | Check | Query | Expected |
   |-------|-------|----------|
   | Isnad completeness | Narrators have relationships | > 95% |
   | Chain traversal | Can follow complete chain | All pass |
   | Source attribution | All hadith linked to collection | 100% |

3. **Graph Relationship Validation**
   ```sql
   -- Find orphan nodes (no relationships)
   SELECT * FROM node WHERE NOT ->relates->any AND NOT <-relates<-any;
   
   -- Verify edge integrity
   SELECT * FROM edge WHERE NOT in OR NOT out;
   
   -- Check for duplicate nodes
   SELECT text, count() 
   FROM node 
   GROUP BY text 
   HAVING count > 1;
   ```

4. **Automated Validation Script**
   ```bash
   # Run data validation suite
   cargo test --package al-mizan --test data_integrity
   
   # Or via API
   curl -X POST http://localhost:8000/api/v1/admin/validate \
     -H "Authorization: Bearer $ADMIN_TOKEN"
   ```

5. **Validation Report Template**
   ```markdown
   ## Data Integrity Report: [Date]
   
   ### Quran Data
   | Check | Expected | Actual | Status |
   |-------|----------|--------|--------|
   | Verse Count | 6,236 | X | ✅/❌ |
   
   ### Hadith Data
   | Check | Expected | Actual | Status |
   |-------|----------|--------|--------|
   
   ### Graph Integrity
   | Check | Expected | Actual | Status |
   |-------|----------|--------|--------|
   
   ### Anomalies Found
   - [List any data issues]
   
   ### Remediation Actions
   - [Steps to fix issues]
   ```

### Output
- Data integrity report
- Issue tickets for any anomalies

---

## 5. Performance Testing Runbook

### Purpose
Validate system performance meets requirements.

### Procedure

1. **Performance Requirements**
   | Metric | Target | Measurement |
   |--------|--------|-------------|
   | API Response (p50) | < 10ms | k6 / Drill |
   | API Response (p99) | < 50ms | k6 / Drill |
   | Throughput | > 1000 RPS | Load test |
   | Error Rate | < 0.1% | Under load |
   | Concurrent Users | 100+ | Stress test |

2. **Test Scenarios**
   ```javascript
   // k6 load test example
   import http from 'k6/http';
   import { check } from 'k6';
   
   export const options = {
     stages: [
       { duration: '30s', target: 50 },  // Ramp up
       { duration: '1m', target: 100 },  // Steady
       { duration: '30s', target: 0 },   // Ramp down
     ],
     thresholds: {
       http_req_duration: ['p(99) < 50'],
       http_req_failed: ['rate < 0.01'],
     },
   };
   
   export default function () {
     const res = http.get('http://localhost:8000/api/v1/quran/1/1');
     check(res, {
       'status is 200': (r) => r.status === 200,
       'response time < 50ms': (r) => r.timings.duration < 50,
     });
   }
   ```

3. **Test Types**
   | Type | Purpose | Duration | Load |
   |------|---------|----------|------|
   | **Baseline** | Establish normal metrics | 5 min | 10 VUs |
   | **Load** | Normal expected load | 10 min | 100 VUs |
   | **Stress** | Find breaking point | 15 min | Ramp to failure |
   | **Spike** | Sudden traffic burst | 5 min | 0→200→0 |
   | **Endurance** | Memory leaks, degradation | 1 hour | 50 VUs |

4. **Execution Checklist**
   - [ ] Verify test environment is isolated
   - [ ] Ensure baseline database state
   - [ ] Run warm-up requests
   - [ ] Execute test scenario
   - [ ] Collect metrics and logs
   - [ ] Compare against baseline

5. **Performance Report Template**
   ```markdown
   ## Performance Test Report: [Date]
   
   **Test Type**: [Load / Stress / Spike / Endurance]
   **Duration**: [Time]
   **Target Load**: [VUs / RPS]
   
   ### Results Summary
   | Metric | Target | Actual | Status |
   |--------|--------|--------|--------|
   | p50 Response | < 10ms | X ms | ✅/❌ |
   | p99 Response | < 50ms | X ms | ✅/❌ |
   | Max RPS | > 1000 | X | ✅/❌ |
   | Error Rate | < 0.1% | X% | ✅/❌ |
   
   ### Bottlenecks Identified
   - [CPU / Memory / DB / Network]
   
   ### Recommendations
   - [Optimization suggestions]
   ```

### Output
- Performance test report
- Grafana dashboard link (if available)
- Recommendations for optimization

---

## 6. Security Testing Runbook

### Purpose
Identify security vulnerabilities in the API and application.

### Procedure

1. **OWASP API Security Top 10 Checks**
   | # | Vulnerability | Test Approach |
   |---|---------------|---------------|
   | 1 | Broken Object Level Auth | Access other users' resources |
   | 2 | Broken Authentication | Token manipulation, brute force |
   | 3 | Excessive Data Exposure | Check response filtering |
   | 4 | Lack of Resources & Rate Limiting | Flood endpoints |
   | 5 | Broken Function Level Auth | Access admin endpoints |
   | 6 | Mass Assignment | Submit unexpected fields |
   | 7 | Security Misconfiguration | Headers, CORS, error messages |
   | 8 | Injection | SQL, NoSQL, command injection |
   | 9 | Improper Assets Management | Deprecated/shadow APIs |
   | 10 | Insufficient Logging | Verify audit trails |

2. **Automated Scanning**
   ```bash
   # OWASP ZAP baseline scan
   docker run -t owasp/zap2docker-stable zap-baseline.py \
     -t http://localhost:8000/api/v1 \
     -r security-report.html
   
   # Dependency vulnerability scan
   cargo audit
   ```

3. **Manual Security Tests**
   - [ ] API key validation (invalid, expired, revoked)
   - [ ] Rate limiting enforcement
   - [ ] Input validation (special characters, SQL injection)
   - [ ] Error message information leakage
   - [ ] HTTPS enforcement
   - [ ] CORS configuration

4. **Security Report Template**
   ```markdown
   ## Security Test Report: [Date]
   
   ### Automated Scan Results
   | Severity | Count | Fixed |
   |----------|-------|-------|
   | Critical | X | |
   | High | X | |
   | Medium | X | |
   | Low | X | |
   
   ### Findings
   #### [Finding Title]
   - **Severity**: [Critical / High / Medium / Low]
   - **Location**: [Endpoint / Component]
   - **Description**: [Details]
   - **Remediation**: [Fix recommendation]
   
   ### Compliance Status
   - [ ] OWASP API Top 10 reviewed
   - [ ] No critical vulnerabilities
   - [ ] Dependency audit clean
   ```

### Output
- Security assessment report
- Vulnerability tickets with remediation priority
- Compliance checklist

---

*This runbook is maintained by the QA Expert Agent. Last updated: 2026-01-04T21:24:00+08:00*
