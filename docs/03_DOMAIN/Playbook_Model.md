# Playbook Model

## Schema

```rust
struct Playbook {
    id: Uuid,
    name: String,
    description: Option<String>,
    trigger: PlaybookTrigger,
    steps: Vec<PlaybookStep>,
    is_active: bool,
    workspace_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

enum PlaybookTrigger {
    Manual,
    OnIncidentCreated { severity_min: Option<Severity> },
    OnTechniqueDetected { techniques: Vec<TechniqueId> },
    OnObservableType { observable_types: Vec<ObservableType> },
    OnSchedule { cron: String },
}

struct PlaybookStep {
    id: Uuid,
    order: i32,
    action: PlaybookAction,
    condition: Option<StepCondition>,
    on_failure: Option<StepFailureAction>,
    timeout_seconds: Option<i32>,
}

enum PlaybookAction {
    SendWebhook { url: String, payload: Value },
    SendEmail { to: Vec<String>, template: String },
    CreateTicket { system: String, template: String },
    RunCommand { command: String, timeout: i32 },
    EnrichObservables { types: Vec<ObservableType> },
    UpdateIncidentStatus { status: IncidentStatus },
    NotifySlack { channel: String, message: String },
    WaitForApproval { assignee: Uuid },
    RunSubPlaybook { playbook_id: Uuid },
}
```

## Execution

Playbooks execute as DAGs (directed acyclic graphs) via a background worker pool. Each step has retry logic with exponential backoff (3 retries, 30s initial backoff).
