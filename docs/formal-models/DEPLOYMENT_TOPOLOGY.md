# Al-Mizan: Formal Deployment Topology

This document strictly models the physical deployment nodes, execution environments, and network boundaries using UML Deployment Diagram syntax.

---

## 1. Physical Topology Diagram

```plantuml
@startuml
' SCALABILITY: Hybrid Cloud + Sovereign Bare Metal
' SECURITY: Zero Trust Network Architecture (ZTNA)

node "Public Cloud Region (AWS/GCP)" as PublicCloud {
    package "DMZ (Public Subnet)" {
        node "Load Balancer" as LB
        node "CDN Edge" as CDN
    }
}

node "Sovereign Bare Metal (Cluster)" as Sovereign {
    package "App Zone (Private Subnet)" {
        node "Kubernetes Worker" as K8s {
            artifact "Synthesis Pod" <<Docker>>
            artifact "RAG Service" <<Docker>>
        }
    }
    
    package "Secure Enclave (TEE)" {
        node "Intel SGX / AMD SEV" as TEE {
            artifact "Consensus Logic" <<WASM>>
            artifact "Signing Service" <<Binary>>
        }
    }
    
    package "Data Zone (Restricted)" {
        database "SurrealDB\n(Graph Store)" as DB
        database "Ledger Storage\n(Merkle)" as Ledger
    }
}

node "Client Device" as Client {
    artifact "Browser / Mobile App" as App
}

' Network Connections
App -- CDN : HTTPS (TLS 1.3)
CDN -- LB : Anycast
LB -- K8s : mTLS (Mesh)

K8s -- TEE : Remote Attestation (gRPC)
K8s -- DB : SurrealQL (WSS)

TEE -- Ledger : Direct IO (Encrypted)

note right of TEE
  **Hardware Root of Trust**
  Code inside here cannot be
  inspected by host root admin.
end note

@enduml
```

---

## 2. Deployment Specifications

### 2.1 Artifact Manifest
| Artifact | Type | Host Node | Security Level |
| :--- | :--- | :--- | :--- |
| **Synthesis Pod** | Docker Image | K8s Worker | `Confidential` |
| **Consensus Logic** | WASM Module | TEE (SGX) | `Strictly Sovereign` |
| **Signing Service** | Binary | TEE (SGX) | `Strictly Sovereign` |
| **Graph Store** | Database File | Data Zone | `Encrypted-at-Rest` |

### 2.2 Network Constraints (OCL)
*   `context Connection inv: self.protocol = 'HTTP' implies self.encryption = 'TLS 1.3'`
*   `context Node inv: self.zone = 'Secure Enclave' implies self.accessControl = 'Remote Attestation'`
