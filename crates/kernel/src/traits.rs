use crate::KernelError;
use std::fmt::Debug;

pub trait IntelligenceObject: Debug + Send + Sync {
    fn id(&self) -> &str;
    fn object_type(&self) -> &'static str;
    fn validate(&self) -> Result<(), KernelError>;
}

pub trait Repository<T: IntelligenceObject>: Debug + Send + Sync {
    fn save(&self, object: T) -> Result<(), KernelError>;
    fn find_by_id(&self, id: &str) -> Result<Option<T>, KernelError>;
    fn delete(&self, id: &str) -> Result<(), KernelError>;
}

pub trait ConfidenceScore: Debug + Send + Sync {
    fn score(&self) -> f64;
    fn propagate(&self, trust: f64) -> Box<dyn ConfidenceScore>;
}

pub trait DomainEvent: Debug + Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> &str;
    fn payload(&self) -> &str;
}
