use crate::{Confidence, KernelError, Provenance};
use std::fmt::Debug;

pub trait IntelligenceObject: Debug + Send + Sync {
    fn id(&self) -> &str;
    fn object_type(&self) -> &'static str;
    fn validate(&self) -> Result<(), KernelError>;
    fn confidence(&self) -> Confidence;
    fn provenance(&self) -> Provenance;
}

pub trait Repository<T: IntelligenceObject>: Debug + Send + Sync {
    fn save(&self, object: T) -> Result<(), KernelError>;
    fn find_by_id(&self, id: &str) -> Result<Option<T>, KernelError>;
    fn delete(&self, id: &str) -> Result<(), KernelError>;
}

pub trait DomainEvent: Debug + Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> &str;
    fn payload(&self) -> &str;
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;
}
