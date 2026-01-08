//! Comprehensive domain tests for Al-Mizan Core
//! 
//! These tests ensure long-term stability of domain logic.

#[cfg(test)]
mod shariah_tests {
    use crate::enterprise::shariah::{analyze_contract, check_standard, Contract};

    #[test]
    fn test_riba_detection() {
        let contract = Contract {
            contract_type: "Murabaha".to_string(),
            rate: "fixed".to_string(),
            late_fee: "compounding".to_string(),
        };

        let result = analyze_contract(&contract);
        assert_eq!(result.status, "HARAM");
        assert!(result.violation.is_some());
        assert!(result.violation.unwrap().contains("Riba"));
    }

    #[test]
    fn test_gharar_detection() {
        let contract = Contract {
            contract_type: "Murabaha".to_string(),
            rate: "variable".to_string(),
            late_fee: "fixed_admin_fee".to_string(),
        };

        let result = analyze_contract(&contract);
        assert_eq!(result.status, "WARNING");
        assert!(result.violation.is_some());
        assert!(result.violation.unwrap().contains("Gharar"));
    }

    #[test]
    fn test_halal_contract() {
        let contract = Contract {
            contract_type: "Murabaha".to_string(),
            rate: "fixed".to_string(),
            late_fee: "fixed_admin_fee".to_string(),
        };

        let result = analyze_contract(&contract);
        assert_eq!(result.status, "HALAL");
        assert!(result.violation.is_none());
    }

    #[test]
    fn test_tawarruq_non_standard() {
        let contract = Contract {
            contract_type: "Tawarruq".to_string(),
            rate: "fixed".to_string(),
            late_fee: "fixed_admin_fee".to_string(),
        };

        let result = check_standard(&contract);
        assert!(!result.certified);
        assert!(result.reason.is_some());
    }

    #[test]
    fn test_gold_standard_certification() {
        let contract = Contract {
            contract_type: "Murabaha".to_string(),
            rate: "fixed".to_string(),
            late_fee: "fixed_admin_fee".to_string(),
        };

        let result = check_standard(&contract);
        assert!(result.certified);
        assert_eq!(result.badge, Some("Al-Mizan Gold".to_string()));
    }

    #[test]
    fn test_silver_standard_certification() {
        let contract = Contract {
            contract_type: "Musharakah".to_string(),
            rate: "profit_share".to_string(),
            late_fee: "none".to_string(),
        };

        let result = check_standard(&contract);
        assert!(result.certified);
        assert_eq!(result.badge, Some("Al-Mizan Silver".to_string()));
    }
}

#[cfg(test)]
mod identity_tests {
    use crate::identity::did::generate_did_key;

    #[test]
    fn test_did_generation() {
        let doc = generate_did_key();
        
        assert!(doc.id.starts_with("did:key:"));
        assert!(!doc.verification_method.is_empty());
        
        let vm = &doc.verification_method[0];
        assert!(vm.id.contains(&doc.id));
        assert_eq!(vm.r#type, "Ed25519VerificationKey2020");
        assert_eq!(vm.controller, doc.id);
    }
}

#[cfg(test)]
mod model_tests {
    use crate::domain::models::*;

    #[test]
    fn test_mutability_serialization() {
        let constant = Mutability::CONSTANT;
        let variable = Mutability::VARIABLE;

        let const_json = serde_json::to_string(&constant).unwrap();
        let var_json = serde_json::to_string(&variable).unwrap();

        assert_eq!(const_json, "\"CONSTANT\"");
        assert_eq!(var_json, "\"VARIABLE\"");
    }

    #[test]
    fn test_grading_serialization() {
        let grades = vec![
            (Grading::Sahih, "\"Sahih\""),
            (Grading::Hasan, "\"Hasan\""),
            (Grading::Daif, "\"Daif\""),
            (Grading::Mawdu, "\"Mawdu\""),
        ];

        for (grade, expected) in grades {
            let json = serde_json::to_string(&grade).unwrap();
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn test_scholar_status_serialization() {
        let statuses = vec![
            (ScholarStatus::Active, "\"Active\""),
            (ScholarStatus::Slashed, "\"Slashed\""),
            (ScholarStatus::Suspended, "\"Suspended\""),
        ];

        for (status, expected) in statuses {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn test_divine_name_serialization() {
        let name = DivineName {
            id: None,
            transliteration: "Ar-Rahman".to_string(),
            arabic: "الرحمن".to_string(),
            meaning_en: "The Most Gracious".to_string(),
        };

        let json = serde_json::to_value(&name).unwrap();
        assert_eq!(json["transliteration"], "Ar-Rahman");
        assert_eq!(json["arabic"], "الرحمن");
    }
}
