use odin_kernel::{Evidence, EvidenceType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStep {
    pub step_type: StepType,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub prerequisites: Vec<String>,
    pub estimated_duration: String,
    pub risk_if_skipped: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepType {
    IsolateNetwork,
    CollectForensics,
    BlockIndicators,
    InvestigateScope,
    EradicateThreat,
    RecoverSystems,
    NotifyStakeholders,
    EscalateToCiso,
    UpdateFirewallRules,
    ResetCredentials,
    HardenEndpoints,
    ReviewLogs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub recommended_steps: Vec<NextStep>,
    pub predicted_attack_phase: String,
    pub estimated_time_to_contain: String,
    pub confidence: f64,
    pub reasoning: String,
}

pub struct StepPredictor;

impl StepPredictor {
    pub fn predict(evidence: &[Evidence], severity: &str, mitre_techniques: &[String]) -> PredictionResult {
        let mut steps = Vec::new();
        let has_network = evidence.iter().any(|e| matches!(e.content_type, EvidenceType::NetworkCapture));
        let has_logs = evidence.iter().any(|e| matches!(e.content_type, EvidenceType::Log));
        let _has_mem_dump = evidence.iter().any(|e| matches!(e.content_type, EvidenceType::MemoryDump));
        let _has_fs_artifact = evidence.iter().any(|e| matches!(e.content_type, EvidenceType::FileSystemArtifact));

        let is_lateral_movement = mitre_techniques.iter().any(|t| t.starts_with("T1021") || t.starts_with("T1570"));
        let is_persistence = mitre_techniques.iter().any(|t| t.starts_with("T1053") || t.starts_with("T1547") || t.starts_with("T1543"));
        let is_c2 = mitre_techniques.iter().any(|t| t.starts_with("T1071") || t.starts_with("T1105") || t.starts_with("T1573"));
        let is_data_exfil = mitre_techniques.iter().any(|t| t.starts_with("T1041") || t.starts_with("T1048") || t.starts_with("T1567"));

        steps.push(NextStep {
            step_type: StepType::CollectForensics,
            title: "Collect forensic evidence".into(),
            description: "Preserve volatile memory, disk images, and network captures for analysis".into(),
            confidence: 0.95,
            prerequisites: vec![],
            estimated_duration: "30-60 minutes".into(),
            risk_if_skipped: RiskLevel::Critical,
        });

        if has_network || is_c2 {
            steps.push(NextStep {
                step_type: StepType::IsolateNetwork,
                title: "Isolate affected systems".into(),
                description: "Disconnect compromised hosts from the network to prevent lateral movement".into(),
                confidence: 0.90,
                prerequisites: vec!["Network topology identified".into()],
                estimated_duration: "5-15 minutes".into(),
                risk_if_skipped: RiskLevel::Critical,
            });
        }

        if is_c2 || has_network {
            steps.push(NextStep {
                step_type: StepType::BlockIndicators,
                title: "Block C2 indicators".into(),
                description: "Add known C2 IPs, domains, and URLs to firewall block lists".into(),
                confidence: 0.85,
                prerequisites: vec!["C2 infrastructure identified".into()],
                estimated_duration: "15-30 minutes".into(),
                risk_if_skipped: RiskLevel::High,
            });
        }

        if is_persistence {
            steps.push(NextStep {
                step_type: StepType::InvestigateScope,
                title: "Investigate persistence mechanisms".into(),
                description: "Search for scheduled tasks, services, registry keys, and startup items".into(),
                confidence: 0.80,
                prerequisites: vec!["Endpoint access available".into()],
                estimated_duration: "1-2 hours".into(),
                risk_if_skipped: RiskLevel::High,
            });
        }

        if is_lateral_movement {
            steps.push(NextStep {
                step_type: StepType::UpdateFirewallRules,
                title: "Update firewall rules".into(),
                description: "Restrict lateral movement paths between network segments".into(),
                confidence: 0.80,
                prerequisites: vec!["Network segmentation map".into()],
                estimated_duration: "30-60 minutes".into(),
                risk_if_skipped: RiskLevel::High,
            });
        }

        if has_logs {
            steps.push(NextStep {
                step_type: StepType::ReviewLogs,
                title: "Review authentication logs".into(),
                description: "Analyze Windows Event Logs, auth logs, and access records for anomalies".into(),
                confidence: 0.75,
                prerequisites: vec!["Log access available".into()],
                estimated_duration: "1-3 hours".into(),
                risk_if_skipped: RiskLevel::Medium,
            });
        }

        if severity == "Critical" || severity == "High" || is_data_exfil {
            steps.push(NextStep {
                step_type: StepType::EscalateToCiso,
                title: "Escalate to CISO".into(),
                description: "Notify CISO and executive team of security incident".into(),
                confidence: 0.85,
                prerequisites: vec![],
                estimated_duration: "5-10 minutes".into(),
                risk_if_skipped: RiskLevel::High,
            });
        }

        if severity == "Critical" || is_data_exfil {
            steps.push(NextStep {
                step_type: StepType::NotifyStakeholders,
                title: "Notify stakeholders".into(),
                description: "Inform legal, compliance, and affected parties per regulatory requirements".into(),
                confidence: 0.70,
                prerequisites: vec!["Impact assessment complete".into()],
                estimated_duration: "1-2 hours".into(),
                risk_if_skipped: RiskLevel::Medium,
            });
        }

        steps.push(NextStep {
            step_type: StepType::EradicateThreat,
            title: "Eradicate threat".into(),
            description: "Remove malware, close backdoors, and eliminate attacker persistence".into(),
            confidence: 0.75,
            prerequisites: vec!["Root cause identified".into(), "IOCs cataloged".into()],
            estimated_duration: "2-4 hours".into(),
            risk_if_skipped: RiskLevel::High,
        });

        steps.push(NextStep {
            step_type: StepType::ResetCredentials,
            title: "Reset compromised credentials".into(),
            description: "Force password resets for all potentially compromised accounts".into(),
            confidence: 0.70,
            prerequisites: vec!["Affected accounts identified".into()],
            estimated_duration: "30-60 minutes".into(),
            risk_if_skipped: RiskLevel::Medium,
        });

        steps.push(NextStep {
            step_type: StepType::RecoverSystems,
            title: "Recover systems".into(),
            description: "Restore systems from clean backups and verify integrity".into(),
            confidence: 0.65,
            prerequisites: vec!["Threat eradicated".into(), "Clean backups available".into()],
            estimated_duration: "4-8 hours".into(),
            risk_if_skipped: RiskLevel::Medium,
        });

        steps.push(NextStep {
            step_type: StepType::HardenEndpoints,
            title: "Harden endpoints".into(),
            description: "Apply patches, update EDR rules, and tighten endpoint security policies".into(),
            confidence: 0.60,
            prerequisites: vec!["Systems recovered".into()],
            estimated_duration: "2-4 hours".into(),
            risk_if_skipped: RiskLevel::Low,
        });

        steps.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        let attack_phase = if is_data_exfil {
            "Exfiltration"
        } else if is_c2 {
            "Command & Control"
        } else if is_lateral_movement {
            "Lateral Movement"
        } else if is_persistence {
            "Persistence"
        } else {
            "Initial Access"
        };

        let avg_conf: f64 = steps.iter().map(|s| s.confidence).sum::<f64>() / steps.len() as f64;

        let time_to_contain = if severity == "Critical" {
            "< 1 hour"
        } else if severity == "High" {
            "1-2 hours"
        } else {
            "2-4 hours"
        };

        PredictionResult {
            recommended_steps: steps,
            predicted_attack_phase: attack_phase.to_string(),
            estimated_time_to_contain: time_to_contain.to_string(),
            confidence: avg_conf,
            reasoning: format!(
                "Analysis of {} evidence items with {} MITRE techniques suggests {} phase. Severity: {}.",
                evidence.len(),
                mitre_techniques.len(),
                attack_phase,
                severity,
            ),
        }
    }
}
