use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub query_hash: String,
    pub result_id: String,
    pub action: String,
    pub reward: f64,
    pub state_features: Vec<f64>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct RLFeedbackLoop {
    q_table: HashMap<String, HashMap<String, f64>>,
    learning_rate: f64,
    discount_factor: f64,
    exploration_rate: f64,
    exploration_decay: f64,
    min_exploration: f64,
    replay_buffer: Vec<Experience>,
    max_buffer_size: usize,
    #[allow(dead_code)]
    batch_size: usize,
    pub total_updates: u64,
    weight_history: Vec<WeightSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightSnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub structural_weight: f64,
    pub semantic_weight: f64,
    pub context_weight: f64,
    pub feedback_weight: f64,
    pub avg_reward: f64,
}

impl RLFeedbackLoop {
    pub fn new() -> Self {
        Self {
            q_table: HashMap::new(),
            learning_rate: 0.1,
            discount_factor: 0.95,
            exploration_rate: 0.3,
            exploration_decay: 0.995,
            min_exploration: 0.05,
            replay_buffer: Vec::new(),
            max_buffer_size: 10000,
            batch_size: 32,
            total_updates: 0,
            weight_history: Vec::new(),
        }
    }

    pub fn record_experience(
        &mut self,
        query_id: &str,
        result_id: &str,
        action: &str,
        reward: f64,
        state_features: Vec<f64>,
    ) {
        let experience = Experience {
            query_hash: query_id.to_string(),
            result_id: result_id.to_string(),
            action: action.to_string(),
            reward,
            state_features,
            timestamp: chrono::Utc::now(),
        };

        self.replay_buffer.push(experience);
        if self.replay_buffer.len() > self.max_buffer_size {
            self.replay_buffer.remove(0);
        }

        self.update_q_value(query_id, action, reward);
        self.total_updates += 1;

        if self.total_updates.is_multiple_of(100) {
            self.decay_exploration();
        }
    }

    fn update_q_value(&mut self, state: &str, action: &str, reward: f64) {
        let state_q = self.q_table.entry(state.to_string()).or_default();
        let current_q = state_q.get(action).copied().unwrap_or(0.0);

        let max_next_q = state_q.values().cloned().fold(f64::NEG_INFINITY, f64::max);
        let max_next_q = if max_next_q == f64::NEG_INFINITY {
            0.0
        } else {
            max_next_q
        };

        let new_q = current_q
            + self.learning_rate * (reward + self.discount_factor * max_next_q - current_q);

        state_q.insert(action.to_string(), new_q);
    }

    fn decay_exploration(&mut self) {
        self.exploration_rate =
            (self.exploration_rate * self.exploration_decay).max(self.min_exploration);
    }

    pub fn should_explore(&self) -> bool {
        rand::random::<f64>() < self.exploration_rate
    }

    pub fn get_q_value(&self, state: &str, action: &str) -> f64 {
        self.q_table
            .get(state)
            .and_then(|actions| actions.get(action))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn get_best_action(&self, state: &str) -> Option<String> {
        self.q_table.get(state).and_then(|actions| {
            actions
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(action, _)| action.clone())
        })
    }

    pub fn compute_adaptive_weights(
        &self,
        structural_score: f64,
        semantic_score: f64,
        context_score: f64,
    ) -> (f64, f64, f64, f64) {
        let _total_updates = self.total_updates.max(1) as f64;
        let recency_bonus = (self.total_updates as f64 / 1000.0).min(1.0);

        let base_structural = 0.35;
        let base_semantic = 0.35;
        let base_context = 0.15;
        let base_feedback = 0.15;

        let feedback_boost = if !self.replay_buffer.is_empty() {
            let recent_rewards: Vec<f64> = self
                .replay_buffer
                .iter()
                .rev()
                .take(100)
                .map(|e| e.reward)
                .collect();
            let avg_reward =
                recent_rewards.iter().sum::<f64>() / recent_rewards.len().max(1) as f64;
            (avg_reward / 5.0).clamp(0.0, 1.0) * 0.1
        } else {
            0.0
        };

        let structural_adj = base_structural + (structural_score - 0.5) * 0.1 * recency_bonus;
        let semantic_adj = base_semantic + (semantic_score - 0.5) * 0.1 * recency_bonus;
        let context_adj = base_context + (context_score - 0.5) * 0.05 * recency_bonus;
        let feedback_adj = base_feedback + feedback_boost;

        let total = structural_adj + semantic_adj + context_adj + feedback_adj;
        (
            structural_adj / total,
            semantic_adj / total,
            context_adj / total,
            feedback_adj / total,
        )
    }

    pub fn compute_reward(
        &self,
        result_relevance: f64,
        user_feedback: Option<f64>,
        click_through: bool,
        time_spent_secs: Option<f64>,
    ) -> f64 {
        let mut reward = 0.0;

        reward += result_relevance * 2.0;

        if let Some(feedback) = user_feedback {
            reward += (feedback / 5.0) * 3.0;
        }

        if click_through {
            reward += 0.5;
        }

        if let Some(time) = time_spent_secs {
            let time_score = if time < 5.0 {
                0.0
            } else if time < 30.0 {
                0.3
            } else if time < 120.0 {
                0.5
            } else {
                0.2
            };
            reward += time_score;
        }

        reward.clamp(0.0, 5.0)
    }

    pub fn get_stats(&self) -> RLStats {
        let avg_reward = if self.replay_buffer.is_empty() {
            0.0
        } else {
            self.replay_buffer.iter().map(|e| e.reward).sum::<f64>()
                / self.replay_buffer.len() as f64
        };

        let recent_rewards: Vec<f64> = self
            .replay_buffer
            .iter()
            .rev()
            .take(100)
            .map(|e| e.reward)
            .collect();
        let recent_avg = if recent_rewards.is_empty() {
            0.0
        } else {
            recent_rewards.iter().sum::<f64>() / recent_rewards.len() as f64
        };

        RLStats {
            total_experiences: self.replay_buffer.len(),
            total_updates: self.total_updates,
            exploration_rate: self.exploration_rate,
            avg_reward,
            recent_avg_reward: recent_avg,
            q_table_size: self.q_table.len(),
        }
    }

    pub fn record_weight_snapshot(&mut self, weights: (f64, f64, f64, f64), avg_reward: f64) {
        self.weight_history.push(WeightSnapshot {
            timestamp: chrono::Utc::now(),
            structural_weight: weights.0,
            semantic_weight: weights.1,
            context_weight: weights.2,
            feedback_weight: weights.3,
            avg_reward,
        });
        if self.weight_history.len() > 1000 {
            self.weight_history.remove(0);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RLStats {
    pub total_experiences: usize,
    pub total_updates: u64,
    pub exploration_rate: f64,
    pub avg_reward: f64,
    pub recent_avg_reward: f64,
    pub q_table_size: usize,
}

impl Default for RLFeedbackLoop {
    fn default() -> Self {
        Self::new()
    }
}
