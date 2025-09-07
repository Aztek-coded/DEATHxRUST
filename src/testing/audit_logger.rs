use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use crate::testing::response_validator::ValidationResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub command_name: String,
    pub test_timestamp: DateTime<Utc>,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub compliance_score: f64,
    pub validation_details: Vec<ValidationSummary>,
    pub test_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub scenario_name: String,
    pub passed: bool,
    pub color_compliant: bool,
    pub format_issues: Vec<String>,
    pub verbosity_score: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCoverage {
    pub command_name: String,
    pub last_tested: Option<DateTime<Utc>>,
    pub test_count: usize,
    pub compliance_status: ComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    FullyCompliant,
    PartiallyCompliant,
    NonCompliant,
    NotTested,
}

pub struct AuditLogger {
    test_results: Vec<TestResult>,
    command_coverage: HashMap<String, CommandCoverage>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            test_results: Vec::new(),
            command_coverage: HashMap::new(),
        }
    }
    
    pub fn log_test_execution(
        &mut self,
        command_name: &str,
        scenarios_count: usize,
        validation_results: &[ValidationResult],
        duration_ms: u64,
    ) -> TestResult {
        let passed_scenarios = validation_results.iter()
            .filter(|r| r.color_compliant && r.format_issues.is_empty())
            .count();
        
        let failed_scenarios = scenarios_count - passed_scenarios;
        let compliance_score = if scenarios_count > 0 {
            (passed_scenarios as f64 / scenarios_count as f64) * 100.0
        } else {
            0.0
        };
        
        let validation_details: Vec<ValidationSummary> = validation_results.iter()
            .enumerate()
            .map(|(i, result)| ValidationSummary {
                scenario_name: format!("Scenario {}", i + 1),
                passed: result.color_compliant && result.format_issues.is_empty(),
                color_compliant: result.color_compliant,
                format_issues: result.format_issues.clone(),
                verbosity_score: format!("{:?}", result.verbosity_score),
            })
            .collect();
        
        let test_result = TestResult {
            command_name: command_name.to_string(),
            test_timestamp: Utc::now(),
            total_scenarios: scenarios_count,
            passed_scenarios,
            failed_scenarios,
            compliance_score,
            validation_details,
            test_duration_ms: duration_ms,
        };
        
        self.test_results.push(test_result.clone());
        
        self.update_command_coverage(command_name, compliance_score);
        
        info!(
            "Test results saved to audit log for {}: {} passed, {} failed, {:.1}% compliance",
            command_name, passed_scenarios, failed_scenarios, compliance_score
        );
        
        test_result
    }
    
    fn update_command_coverage(&mut self, command_name: &str, compliance_score: f64) {
        let compliance_status = match compliance_score {
            s if s >= 100.0 => ComplianceStatus::FullyCompliant,
            s if s >= 75.0 => ComplianceStatus::PartiallyCompliant,
            s if s > 0.0 => ComplianceStatus::NonCompliant,
            _ => ComplianceStatus::NotTested,
        };
        
        let coverage = self.command_coverage
            .entry(command_name.to_string())
            .or_insert_with(|| CommandCoverage {
                command_name: command_name.to_string(),
                last_tested: None,
                test_count: 0,
                compliance_status: ComplianceStatus::NotTested,
            });
        
        coverage.last_tested = Some(Utc::now());
        coverage.test_count += 1;
        coverage.compliance_status = compliance_status;
    }
    
    pub fn get_command_coverage(&self) -> Vec<CommandCoverage> {
        self.command_coverage.values().cloned().collect()
    }
    
    pub fn calculate_overall_coverage(&self, total_commands: usize) -> f64 {
        let tested_commands = self.command_coverage.len();
        if total_commands > 0 {
            (tested_commands as f64 / total_commands as f64) * 100.0
        } else {
            0.0
        }
    }
    
    pub fn get_recent_test_results(&self, limit: usize) -> Vec<&TestResult> {
        self.test_results
            .iter()
            .rev()
            .take(limit)
            .collect()
    }
    
    pub fn get_test_history_for_command(&self, command_name: &str) -> Vec<&TestResult> {
        self.test_results
            .iter()
            .filter(|r| r.command_name == command_name)
            .collect()
    }
    
    pub fn export_audit_report(&self) -> String {
        let mut report = String::from("=== Command Response Test Audit Report ===\n\n");
        
        report.push_str(&format!("Total Tests Executed: {}\n", self.test_results.len()));
        report.push_str(&format!("Commands Tested: {}\n\n", self.command_coverage.len()));
        
        report.push_str("Command Coverage:\n");
        for coverage in self.command_coverage.values() {
            report.push_str(&format!(
                "  - {}: {:?} (tested {} times, last: {:?})\n",
                coverage.command_name,
                coverage.compliance_status,
                coverage.test_count,
                coverage.last_tested
            ));
        }
        
        report.push_str("\nRecent Test Results:\n");
        for result in self.get_recent_test_results(5) {
            report.push_str(&format!(
                "  - {} @ {}: {}/{} passed ({:.1}% compliance)\n",
                result.command_name,
                result.test_timestamp.format("%Y-%m-%d %H:%M:%S"),
                result.passed_scenarios,
                result.total_scenarios,
                result.compliance_score
            ));
        }
        
        info!("Audit report generated");
        report
    }
}