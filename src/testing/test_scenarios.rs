use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub enum TestScenario {
    Success {
        params: HashMap<String, String>,
        description: String,
    },
    MissingParam {
        param_name: String,
        description: String,
    },
    InvalidParam {
        param_name: String,
        value: String,
        description: String,
    },
    PermissionDenied {
        description: String,
    },
    RateLimit {
        description: String,
    },
    DatabaseError {
        description: String,
    },
    Custom {
        name: String,
        params: HashMap<String, String>,
        description: String,
    },
}

impl TestScenario {
    pub fn name(&self) -> &str {
        match self {
            Self::Success { .. } => "Success",
            Self::MissingParam { .. } => "Missing Parameter",
            Self::InvalidParam { .. } => "Invalid Parameter",
            Self::PermissionDenied { .. } => "Permission Denied",
            Self::RateLimit { .. } => "Rate Limited",
            Self::DatabaseError { .. } => "Database Error",
            Self::Custom { name, .. } => name,
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            Self::Success { description, .. } |
            Self::MissingParam { description, .. } |
            Self::InvalidParam { description, .. } |
            Self::PermissionDenied { description } |
            Self::RateLimit { description } |
            Self::DatabaseError { description } |
            Self::Custom { description, .. } => description,
        }
    }
}

pub struct ScenarioGenerator {
    command_name: String,
}

impl ScenarioGenerator {
    pub fn new(command_name: impl Into<String>) -> Self {
        Self {
            command_name: command_name.into(),
        }
    }
    
    pub fn generate_basic_scenarios(&self) -> Vec<TestScenario> {
        let mut scenarios = Vec::new();
        
        scenarios.push(TestScenario::Success {
            params: HashMap::new(),
            description: format!("Valid execution of {}", self.command_name),
        });
        
        scenarios.push(TestScenario::PermissionDenied {
            description: format!("User lacks permission to execute {}", self.command_name),
        });
        
        scenarios.push(TestScenario::RateLimit {
            description: format!("Command {} rate limited", self.command_name),
        });
        
        debug!("Generated {} basic test scenarios for {}", scenarios.len(), self.command_name);
        scenarios
    }
    
    pub fn generate_ping_scenarios(&self) -> Vec<TestScenario> {
        vec![
            TestScenario::Success {
                params: HashMap::new(),
                description: "Basic ping command execution".to_string(),
            },
            TestScenario::Custom {
                name: "High Latency".to_string(),
                params: HashMap::from([("latency".to_string(), "500".to_string())]),
                description: "Ping with simulated high latency".to_string(),
            },
        ]
    }
    
    pub fn generate_help_scenarios(&self) -> Vec<TestScenario> {
        vec![
            TestScenario::Success {
                params: HashMap::new(),
                description: "Help without specific command".to_string(),
            },
            TestScenario::Success {
                params: HashMap::from([("command".to_string(), "ping".to_string())]),
                description: "Help for specific command".to_string(),
            },
            TestScenario::InvalidParam {
                param_name: "command".to_string(),
                value: "nonexistent".to_string(),
                description: "Help for non-existent command".to_string(),
            },
        ]
    }
    
    pub fn generate_info_scenarios(&self) -> Vec<TestScenario> {
        vec![
            TestScenario::Success {
                params: HashMap::new(),
                description: "Basic info command execution".to_string(),
            },
            TestScenario::Custom {
                name: "No Guild".to_string(),
                params: HashMap::new(),
                description: "Info command in DM context".to_string(),
            },
        ]
    }
    
    pub fn generate_for_command(&self, command_name: &str) -> Vec<TestScenario> {
        let scenarios = match command_name {
            "ping" => self.generate_ping_scenarios(),
            "help" => self.generate_help_scenarios(),
            "info" => self.generate_info_scenarios(),
            _ => self.generate_basic_scenarios(),
        };
        
        debug!("Generated {} test scenarios for command: {}", scenarios.len(), command_name);
        scenarios
    }
    
    pub fn add_custom_scenario(&self, scenarios: &mut Vec<TestScenario>, scenario: TestScenario) {
        debug!("Adding custom scenario: {}", scenario.name());
        scenarios.push(scenario);
    }
    
    pub fn generate_comprehensive_suite(&self, command_name: &str) -> Vec<TestScenario> {
        let mut scenarios = self.generate_for_command(command_name);
        
        scenarios.push(TestScenario::DatabaseError {
            description: format!("Database connection error during {} execution", command_name),
        });
        
        debug!("Generated comprehensive test suite with {} scenarios", scenarios.len());
        scenarios
    }
}