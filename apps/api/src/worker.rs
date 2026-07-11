use std::sync::Arc;
use tokio::sync::mpsc;

#[allow(dead_code)]
pub enum WorkerTask {
    Consolidation,
    Custom(String),
}

#[allow(dead_code)]
pub struct BackgroundWorker {
    tx: mpsc::Sender<WorkerTask>,
}

impl BackgroundWorker {
    pub fn spawn(state: Arc<crate::state::AppState>) -> Self {
        let (tx, mut rx) = mpsc::channel::<WorkerTask>(32);

        tokio::spawn(async move {
            let mut consolidation_interval = tokio::time::interval(
                std::time::Duration::from_secs(
                    std::env::var("ODIN_CONSOLIDATION_INTERVAL_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(3600)
                )
            );
            consolidation_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

            let mut metrics = WorkerMetrics::default();

            loop {
                tokio::select! {
                    _ = consolidation_interval.tick() => {
                        Self::run_consolidation(&state, &mut metrics).await;
                    }
                    task = rx.recv() => {
                        match task {
                            Some(WorkerTask::Consolidation) => {
                                Self::run_consolidation(&state, &mut metrics).await;
                            }
                            Some(WorkerTask::Custom(name)) => {
                                tracing::info!("Running custom worker task: {}", name);
                            }
                            None => {
                                tracing::info!("Worker channel closed, shutting down");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Self { tx }
    }

    async fn run_consolidation(
        state: &Arc<crate::state::AppState>,
        metrics: &mut WorkerMetrics,
    ) {
        tracing::info!("Worker: running memory consolidation...");
        let start = std::time::Instant::now();

        match state.memory.run_consolidation() {
            Ok(report) => {
                let elapsed = start.elapsed();
                if !report.is_empty() {
                    tracing::info!(
                        "Consolidation complete in {:?}: {} expired, {} pruned, {} consolidated",
                        elapsed,
                        report.expired_count,
                        report.pruned_version_count,
                        report.consolidated_memories.len(),
                    );
                }
                metrics.consolidation_runs += 1;
                metrics.total_expired += report.expired_count;
                metrics.total_pruned += report.pruned_version_count;
                metrics.total_consolidated += report.consolidated_memories.len();
                metrics.last_consolidation = Some(chrono::Utc::now());
            }
            Err(e) => {
                tracing::error!("Consolidation failed: {}", e);
                metrics.consolidation_failures += 1;
            }
        }
    }

    #[allow(dead_code)]
    pub async fn trigger_consolidation(&self) {
        let _ = self.tx.send(WorkerTask::Consolidation).await;
    }

    #[allow(dead_code)]
    pub async fn trigger_custom(&self, name: &str) {
        let _ = self.tx.send(WorkerTask::Custom(name.to_string())).await;
    }
}

#[derive(Debug, Default)]
pub struct WorkerMetrics {
    pub consolidation_runs: u64,
    pub consolidation_failures: u64,
    pub total_expired: usize,
    pub total_pruned: usize,
    pub total_consolidated: usize,
    pub last_consolidation: Option<chrono::DateTime<chrono::Utc>>,
}
