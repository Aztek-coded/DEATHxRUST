pub mod mock_context;
pub mod response_validator;
pub mod test_scenarios;
pub mod audit_logger;

pub use mock_context::{MockContext, MockContextBuilder};
pub use response_validator::{ResponseValidator, ValidationResult, VerbosityScore, EmbedData};
pub use test_scenarios::{TestScenario, ScenarioGenerator};
pub use audit_logger::{AuditLogger, TestResult};

use tracing::info;

pub fn init() {
    info!("Initializing testing framework");
}