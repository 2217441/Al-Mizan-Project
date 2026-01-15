# Al-Mizan: Formal Threat Model (STRIDE)

This document formally models the security architecture using a Data Flow Diagram (DFD) and identifies threats using the STRIDE methodology.

---

## 1. Data Flow Diagram (DFD)

**Scope**: Trust Boundaries between User, DMZ, App Zone, and Secure Enclave.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

Person(hacker, "Attacker", "Malicious Actor")
Person(user, "User", "Legitimate User")

Boundary(public, "Public Internet") {
    System_Ext(cdn, "CDN", "Edge Cache")
}

Boundary(dmz, "DMZ (Trust Level 1)") {
    Container(lb, "Load Balancer", "Nginx", "Termination Point")
}

Boundary(app, "App Zone (Trust Level 2)") {
    Container(api, "API Gateway", "Kong", "Policy Enforcement")
    Container(svc, "Core Service", "Rust", "Business Logic")
}

Boundary(tee, "Secure Enclave (Trust Level 3)") {
    Container(signer, "Key Manager", "SGX", "Key Storage")
}

Rel(hacker, lb, "DDoS / Injection")
Rel(user, cdn, "HTTPS")
Rel(cdn, lb, "HTTPS")

Rel(lb, api, "mTLS")
Rel(api, svc, "gRPC")

Rel(svc, signer, "Remote Attestation Request")
Rel(signer, svc, "Signed Artifact")

@enduml
```

---

## 2. STRIDE Analysis Matrix

| Element | Threat Category | Description | Mitigation |
| :--- | :--- | :--- | :--- |
| **Load Balancer** | **D**enial of Service | Volumetric attack on public IP. | Cloudflare/AWS Shield (L3/L4 Protection). |
| **API Gateway** | **S**poofing | Attacker impersonates a valid user. | JWT Verification signed by Identity Provider. |
| **Core Service** | **T**ampering | Modifying logic in transit. | mTLS between Gateway and Service. |
| **Secure Enclave** | **R**epudiation | Admin denies signing an action. | Immutable Audit Log (Merkle Tree) in TEE. |
| **Database** | **I**nformation Disclosure | Leaking Fatwa drafts. | At-rest encryption + Row-Level Security (RLS). |
| **Admin Panel** | **E**levation of Privilege | User gaining root access. | Hardware 2FA (YubiKey) + RBAC Policies. |
