use crate::store::MemoryStore;
use crate::version::MemoryVersion;
use odin_kernel::{CanonicalIncident, Entity, Evidence, KernelError, KnowledgeObject, KnowledgeStatus, KnowledgeType, MemoryObject};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct PgStore {
    pool: PgPool,
}

impl PgStore {
    pub async fn connect(database_url: &str) -> Result<Self, KernelError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .map_err(|e| KernelError::Internal(format!("DB connection failed: {}", e)))?;
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), KernelError> {
        let sql = include_str!("../migrations/001_initial.sql");
        sqlx::query(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Migration failed: {}", e)))?;

        let sql2 = include_str!("../migrations/002_knowledge_objects.sql");
        sqlx::query(sql2)
            .execute(&self.pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Migration 2 failed: {}", e)))?;

        tracing::info!("Database migrations applied");
        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn save_incident(&self, incident: &CanonicalIncident) -> Result<(), KernelError> {
        sqlx::query(
            r#"INSERT INTO incidents (id, title, description, severity, status, tags, mitre_techniques, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               ON CONFLICT (id) DO UPDATE SET
               title = EXCLUDED.title, description = EXCLUDED.description,
               severity = EXCLUDED.severity, status = EXCLUDED.status,
               tags = EXCLUDED.tags, mitre_techniques = EXCLUDED.mitre_techniques,
               updated_at = EXCLUDED.updated_at"#,
        )
        .bind(&incident.id)
        .bind(&incident.title)
        .bind(&incident.description)
        .bind(format!("{:?}", incident.severity))
        .bind(format!("{:?}", incident.status))
        .bind(&incident.tags)
        .bind(&incident.mitre_techniques)
        .bind(incident.created_at)
        .bind(incident.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Save incident failed: {}", e)))?;
        Ok(())
    }

    pub async fn get_incident(&self, id: &str) -> Result<Option<CanonicalIncident>, KernelError> {
        let row = sqlx::query(
            r#"SELECT id, title, description, severity, status, tags, mitre_techniques, created_at, updated_at
               FROM incidents WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Get incident failed: {}", e)))?;

        Ok(row.map(|r| {
            let severity: String = r.get("severity");
            let status: String = r.get("status");
            CanonicalIncident {
                id: r.get("id"),
                title: r.get("title"),
                description: r.get("description"),
                severity: parse_severity(&severity),
                status: parse_status(&status),
                tags: r.get("tags"),
                mitre_techniques: r.get("mitre_techniques"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                evidence_ids: Vec::new(),
                entity_ids: Vec::new(),
            }
        }))
    }

    pub async fn list_incidents(&self) -> Result<Vec<CanonicalIncident>, KernelError> {
        let rows = sqlx::query(
            r#"SELECT id, title, description, severity, status, tags, mitre_techniques, created_at, updated_at
               FROM incidents ORDER BY created_at DESC"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("List incidents failed: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let severity: String = r.get("severity");
                let status: String = r.get("status");
                CanonicalIncident {
                    id: r.get("id"),
                    title: r.get("title"),
                    description: r.get("description"),
                    severity: parse_severity(&severity),
                    status: parse_status(&status),
                    tags: r.get("tags"),
                    mitre_techniques: r.get("mitre_techniques"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                    evidence_ids: Vec::new(),
                    entity_ids: Vec::new(),
                }
            })
            .collect())
    }

    pub async fn save_evidence_batch(
        &self,
        incident_id: &str,
        evidence: &[Evidence],
    ) -> Result<(), KernelError> {
        if evidence.is_empty() {
            return Ok(());
        }
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| KernelError::Internal(format!("Tx begin failed: {}", e)))?;
        for e in evidence {
            sqlx::query(
                r#"INSERT INTO evidence (id, incident_id, source, content, content_type, trust_score, collected_at, created_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                   ON CONFLICT (id) DO NOTHING"#,
            )
            .bind(&e.id)
            .bind(incident_id)
            .bind(&e.source)
            .bind(&e.content)
            .bind(format!("{:?}", e.content_type))
            .bind(e.trust_score)
            .bind(e.collected_at)
            .bind(e.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| KernelError::Internal(format!("Save evidence failed: {}", e)))?;
        }
        tx.commit()
            .await
            .map_err(|e| KernelError::Internal(format!("Tx commit failed: {}", e)))?;
        Ok(())
    }

    pub async fn get_evidence(
        &self,
        incident_id: &str,
    ) -> Result<Vec<Evidence>, KernelError> {
        let rows = sqlx::query(
            r#"SELECT id, incident_id, source, content, content_type, trust_score, collected_at, created_at
               FROM evidence WHERE incident_id = $1 ORDER BY collected_at"#,
        )
        .bind(incident_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Get evidence failed: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let ct: String = r.get("content_type");
                Evidence {
                    id: r.get("id"),
                    incident_id: r.get("incident_id"),
                    source: r.get("source"),
                    content: r.get("content"),
                    content_type: odin_kernel::EvidenceType::Other(ct),
                    trust_score: r.get("trust_score"),
                    collected_at: r.get("collected_at"),
                    created_at: r.get("created_at"),
                }
            })
            .collect())
    }

    pub async fn save_entities_batch(
        &self,
        incident_id: &str,
        entities: &[Entity],
    ) -> Result<(), KernelError> {
        if entities.is_empty() {
            return Ok(());
        }
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| KernelError::Internal(format!("Tx begin failed: {}", e)))?;
        for ent in entities {
            sqlx::query(
                r#"INSERT INTO entities (id, incident_id, name, entity_type, metadata, created_at)
                   VALUES ($1, $2, $3, $4, $5, $6)
                   ON CONFLICT (id) DO NOTHING"#,
            )
            .bind(&ent.id)
            .bind(incident_id)
            .bind(&ent.name)
            .bind(format!("{:?}", ent.entity_type))
            .bind(&ent.metadata)
            .bind(ent.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| KernelError::Internal(format!("Save entity failed: {}", e)))?;
        }
        tx.commit()
            .await
            .map_err(|e| KernelError::Internal(format!("Tx commit failed: {}", e)))?;
        Ok(())
    }

    pub async fn save_feedback(
        &self,
        incident_id: &str,
        feedback_text: &str,
        rating: u8,
    ) -> Result<(), KernelError> {
        sqlx::query(
            r#"INSERT INTO feedback (incident_id, feedback, rating, created_at)
               VALUES ($1, $2, $3, NOW())"#,
        )
        .bind(incident_id)
        .bind(feedback_text)
        .bind(rating as i32)
        .execute(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Save feedback failed: {}", e)))?;
        Ok(())
    }

    pub async fn get_feedback_for_incident(
        &self,
        incident_id: &str,
    ) -> Result<Vec<(String, i32)>, KernelError> {
        let rows = sqlx::query(
            r#"SELECT feedback, rating FROM feedback
               WHERE incident_id = $1 ORDER BY created_at DESC"#,
        )
        .bind(incident_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Get feedback failed: {}", e)))?;

        Ok(rows.iter().map(|r| {
            let feedback: String = r.get("feedback");
            let rating: i32 = r.get("rating");
            (feedback, rating)
        }).collect())
    }

    pub async fn get_average_rating(
        &self,
        incident_id: &str,
    ) -> Result<Option<f64>, KernelError> {
        let row = sqlx::query_scalar::<_, Option<f64>>(
            r#"SELECT AVG(rating::float) FROM feedback WHERE incident_id = $1"#,
        )
        .bind(incident_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Get avg rating failed: {}", e)))?;
        Ok(row.flatten())
    }

    pub async fn get_entities(
        &self,
        incident_id: &str,
    ) -> Result<Vec<Entity>, KernelError> {
        let rows = sqlx::query(
            r#"SELECT id, incident_id, name, entity_type, metadata, created_at
               FROM entities WHERE incident_id = $1"#,
        )
        .bind(incident_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Get entities failed: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let et: String = r.get("entity_type");
                Entity {
                    id: r.get("id"),
                    name: r.get("name"),
                    entity_type: odin_kernel::EntityType::Other(et),
                    metadata: r.get("metadata"),
                    created_at: r.get("created_at"),
                }
            })
            .collect())
    }

    pub async fn update_incident_status(
        &self,
        incident_id: &str,
        status: &str,
    ) -> Result<(), KernelError> {
        sqlx::query(
            r#"UPDATE incidents SET status = $1, updated_at = NOW() WHERE id = $2"#,
        )
        .bind(status)
        .bind(incident_id)
        .execute(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Update status failed: {}", e)))?;
        Ok(())
    }

    pub async fn get_memory_version_counts(
        &self,
    ) -> Result<std::collections::HashMap<String, usize>, KernelError> {
        let rows = sqlx::query(
            r#"SELECT memory_id, COUNT(*) as cnt FROM memory_versions GROUP BY memory_id"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Get version counts failed: {}", e)))?;

        let mut counts = std::collections::HashMap::new();
        for r in rows {
            let mid: String = r.get("memory_id");
            let cnt: i64 = r.get("cnt");
            counts.insert(mid, cnt as usize);
        }
        Ok(counts)
    }
}

impl MemoryStore for PgStore {
    fn save(&self, memory: MemoryObject) -> Result<(), KernelError> {
        let pool = self.pool.clone();
        let m = memory.clone();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            sqlx::query(
                r#"INSERT INTO memories (id, incident_id, summary, context, confidence, version, created_at, expires_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                   ON CONFLICT (id) DO UPDATE SET
                   summary = EXCLUDED.summary, context = EXCLUDED.context,
                   confidence = EXCLUDED.confidence, version = EXCLUDED.version"#,
            )
            .bind(&m.id)
            .bind(&m.incident_id)
            .bind(&m.summary)
            .bind(&m.context)
            .bind(m.confidence)
            .bind(m.version as i64)
            .bind(m.created_at)
            .bind(m.expires_at)
            .execute(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Save memory failed: {}", e)))?;
            Ok(())
        })
    }

    fn find_by_id(&self, id: &str) -> Result<Option<MemoryObject>, KernelError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let row = sqlx::query(
                r#"SELECT id, incident_id, summary, context, confidence, version, created_at, expires_at
                   FROM memories WHERE id = $1"#,
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Find memory failed: {}", e)))?;

            Ok(row.map(|r| {
                let version: i64 = r.get("version");
                MemoryObject {
                    id: r.get("id"),
                    incident_id: r.get("incident_id"),
                    summary: r.get("summary"),
                    context: r.get("context"),
                    confidence: r.get("confidence"),
                    version: version as u64,
                    created_at: r.get("created_at"),
                    expires_at: r.get("expires_at"),
                }
            }))
        })
    }

    fn find_by_incident_id(&self, incident_id: &str) -> Result<Option<MemoryObject>, KernelError> {
        let pool = self.pool.clone();
        let iid = incident_id.to_string();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let row = sqlx::query(
                r#"SELECT id, incident_id, summary, context, confidence, version, created_at, expires_at
                   FROM memories WHERE incident_id = $1"#,
            )
            .bind(&iid)
            .fetch_optional(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Find memory by incident failed: {}", e)))?;

            Ok(row.map(|r| {
                let version: i64 = r.get("version");
                MemoryObject {
                    id: r.get("id"),
                    incident_id: r.get("incident_id"),
                    summary: r.get("summary"),
                    context: r.get("context"),
                    confidence: r.get("confidence"),
                    version: version as u64,
                    created_at: r.get("created_at"),
                    expires_at: r.get("expires_at"),
                }
            }))
        })
    }

    fn list_all(&self) -> Result<Vec<MemoryObject>, KernelError> {
        let pool = self.pool.clone();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let rows = sqlx::query(
                r#"SELECT id, incident_id, summary, context, confidence, version, created_at, expires_at
                   FROM memories ORDER BY created_at DESC"#,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("List memories failed: {}", e)))?;

            Ok(rows
                .into_iter()
                .map(|r| {
                    let version: i64 = r.get("version");
                    MemoryObject {
                        id: r.get("id"),
                        incident_id: r.get("incident_id"),
                        summary: r.get("summary"),
                        context: r.get("context"),
                        confidence: r.get("confidence"),
                        version: version as u64,
                        created_at: r.get("created_at"),
                        expires_at: r.get("expires_at"),
                    }
                })
                .collect())
        })
    }

    fn save_version(&self, version: MemoryVersion) -> Result<(), KernelError> {
        let pool = self.pool.clone();
        let v = version.clone();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let snapshot = serde_json::to_value(&v.memory)
                .map_err(|e| KernelError::Internal(format!("Serialize failed: {}", e)))?;
            sqlx::query(
                r#"INSERT INTO memory_versions (memory_id, version, snapshot, changelog, created_at)
                   VALUES ($1, $2, $3, $4, $5)"#,
            )
            .bind(&v.memory.id)
            .bind(v.version as i64)
            .bind(&snapshot)
            .bind(&v.changelog)
            .bind(v.created_at)
            .execute(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Save version failed: {}", e)))?;
            Ok(())
        })
    }

    fn get_versions(&self, memory_id: &str) -> Result<Vec<MemoryVersion>, KernelError> {
        let pool = self.pool.clone();
        let mid = memory_id.to_string();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let rows = sqlx::query(
                r#"SELECT memory_id, version, snapshot, changelog, created_at
                   FROM memory_versions WHERE memory_id = $1 ORDER BY version"#,
            )
            .bind(&mid)
            .fetch_all(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Get versions failed: {}", e)))?;

            let mut versions = Vec::new();
            for r in rows {
                let version: i64 = r.get("version");
                let snapshot: serde_json::Value = r.get("snapshot");
                let memory: MemoryObject = serde_json::from_value(snapshot)
                    .map_err(|e| KernelError::Internal(format!("Deserialize failed: {}", e)))?;
                versions.push(MemoryVersion {
                    version: version as u64,
                    memory,
                    created_at: r.get("created_at"),
                    changelog: r.get("changelog"),
                });
            }
            Ok(versions)
        })
    }

    fn purge_expired(&self) -> Result<Vec<String>, KernelError> {
        let pool = self.pool.clone();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let rows = sqlx::query(
                r#"DELETE FROM memories WHERE expires_at IS NOT NULL AND expires_at <= NOW()
                   RETURNING id"#,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Purge expired failed: {}", e)))?;
            let ids: Vec<String> = rows.iter().map(|r| r.get("id")).collect();
            if !ids.is_empty() {
                let deleted = ids.len();
                tracing::info!("Purged {} expired memories", deleted);
            }
            Ok(ids)
        })
    }

    fn prune_versions(&self, memory_id: &str, max_versions: usize) -> Result<usize, KernelError> {
        let pool = self.pool.clone();
        let mid = memory_id.to_string();
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| KernelError::Internal("No tokio runtime".into()))?;
        rt.block_on(async move {
            let count_row = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(*) FROM memory_versions WHERE memory_id = $1"#,
            )
            .bind(&mid)
            .fetch_one(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Count versions failed: {}", e)))?;
            let count = count_row as usize;
            if count <= max_versions {
                return Ok(0);
            }
            let to_remove = count - max_versions;
            sqlx::query(
                r#"DELETE FROM memory_versions
                   WHERE memory_id = $1
                   AND id IN (
                       SELECT id FROM memory_versions
                       WHERE memory_id = $1
                       ORDER BY version ASC
                       LIMIT $2
                   )"#,
            )
            .bind(&mid)
            .bind(to_remove as i64)
            .execute(&pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Prune versions failed: {}", e)))?;
            tracing::info!("Pruned {} old versions for memory {}", to_remove, memory_id);
            Ok(to_remove)
        })
    }
}

impl PgStore {
    pub async fn save_knowledge_object(&self, obj: &KnowledgeObject) -> Result<(), KernelError> {
        sqlx::query(
            r#"INSERT INTO knowledge_objects (id, title, description, content, object_type, status, tags, source_incidents, mitre_techniques, confidence_sources, created_by, updated_by, created_at, updated_at, status_history, expires_at, review_notes)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
               ON CONFLICT (id) DO UPDATE SET
               title = EXCLUDED.title, description = EXCLUDED.description,
               content = EXCLUDED.content, status = EXCLUDED.status,
               tags = EXCLUDED.tags, mitre_techniques = EXCLUDED.mitre_techniques,
               updated_by = EXCLUDED.updated_by, updated_at = EXCLUDED.updated_at,
               status_history = EXCLUDED.status_history, review_notes = EXCLUDED.review_notes"#,
        )
        .bind(&obj.id)
        .bind(&obj.title)
        .bind(&obj.description)
        .bind(&obj.content)
        .bind(format!("{:?}", obj.object_type))
        .bind(format!("{:?}", obj.status))
        .bind(&obj.tags)
        .bind(&obj.source_incidents)
        .bind(&obj.mitre_techniques)
        .bind(serde_json::to_value(&obj.confidence_sources).unwrap_or_default())
        .bind(&obj.created_by)
        .bind(&obj.updated_by)
        .bind(obj.created_at)
        .bind(obj.updated_at)
        .bind(serde_json::to_value(&obj.status_history).unwrap_or_default())
        .bind(obj.expires_at)
        .bind(&obj.review_notes)
        .execute(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Save knowledge object failed: {}", e)))?;
        Ok(())
    }

    pub async fn get_knowledge_object(&self, id: &str) -> Result<Option<KnowledgeObject>, KernelError> {
        let row = sqlx::query("SELECT * FROM knowledge_objects WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Get knowledge object failed: {}", e)))?;

        Ok(row.map(|r| KnowledgeObject {
            id: r.get("id"),
            title: r.get("title"),
            description: r.get("description"),
            content: r.get("content"),
            object_type: parse_knowledge_type(&r.get::<String, _>("object_type")),
            status: parse_knowledge_status(&r.get::<String, _>("status")),
            tags: r.get("tags"),
            source_incidents: r.get("source_incidents"),
            mitre_techniques: r.get("mitre_techniques"),
            confidence_sources: serde_json::from_value(r.get("confidence_sources")).unwrap_or_default(),
            created_by: r.get("created_by"),
            updated_by: r.get("updated_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            status_history: serde_json::from_value(r.get("status_history")).unwrap_or_default(),
            expires_at: r.get("expires_at"),
            review_notes: r.get("review_notes"),
        }))
    }

    pub async fn list_knowledge_objects(
        &self,
        status_filter: Option<&str>,
        type_filter: Option<&str>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<KnowledgeObject>, KernelError> {
        let mut query = String::from("SELECT * FROM knowledge_objects WHERE 1=1");
        if let Some(s) = status_filter {
            query.push_str(&format!(" AND status = '{}'", s));
        }
        if let Some(t) = type_filter {
            query.push_str(&format!(" AND object_type = '{}'", t));
        }
        query.push_str(&format!(" ORDER BY updated_at DESC LIMIT {} OFFSET {}", limit, offset));

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| KernelError::Internal(format!("List knowledge objects failed: {}", e)))?;

        Ok(rows.into_iter().map(|r| KnowledgeObject {
            id: r.get("id"),
            title: r.get("title"),
            description: r.get("description"),
            content: r.get("content"),
            object_type: parse_knowledge_type(&r.get::<String, _>("object_type")),
            status: parse_knowledge_status(&r.get::<String, _>("status")),
            tags: r.get("tags"),
            source_incidents: r.get("source_incidents"),
            mitre_techniques: r.get("mitre_techniques"),
            confidence_sources: serde_json::from_value(r.get("confidence_sources")).unwrap_or_default(),
            created_by: r.get("created_by"),
            updated_by: r.get("updated_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            status_history: serde_json::from_value(r.get("status_history")).unwrap_or_default(),
            expires_at: r.get("expires_at"),
            review_notes: r.get("review_notes"),
        }).collect())
    }

    pub async fn delete_knowledge_object(&self, id: &str) -> Result<(), KernelError> {
        sqlx::query("DELETE FROM knowledge_objects WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| KernelError::Internal(format!("Delete knowledge object failed: {}", e)))?;
        Ok(())
    }

    pub async fn search_knowledge_objects(&self, query_text: &str, limit: usize) -> Result<Vec<KnowledgeObject>, KernelError> {
        let rows = sqlx::query(
            r#"SELECT * FROM knowledge_objects
               WHERE title ILIKE $1 OR description ILIKE $1 OR content ILIKE $1
               AND status != 'Purged'
               ORDER BY updated_at DESC LIMIT $2"#,
        )
        .bind(format!("%{}%", query_text))
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| KernelError::Internal(format!("Search knowledge objects failed: {}", e)))?;

        Ok(rows.into_iter().map(|r| KnowledgeObject {
            id: r.get("id"),
            title: r.get("title"),
            description: r.get("description"),
            content: r.get("content"),
            object_type: parse_knowledge_type(&r.get::<String, _>("object_type")),
            status: parse_knowledge_status(&r.get::<String, _>("status")),
            tags: r.get("tags"),
            source_incidents: r.get("source_incidents"),
            mitre_techniques: r.get("mitre_techniques"),
            confidence_sources: serde_json::from_value(r.get("confidence_sources")).unwrap_or_default(),
            created_by: r.get("created_by"),
            updated_by: r.get("updated_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            status_history: serde_json::from_value(r.get("status_history")).unwrap_or_default(),
            expires_at: r.get("expires_at"),
            review_notes: r.get("review_notes"),
        }).collect())
    }
}

fn parse_severity(s: &str) -> odin_kernel::Severity {
    match s {
        "Critical" => odin_kernel::Severity::Critical,
        "High" => odin_kernel::Severity::High,
        "Medium" => odin_kernel::Severity::Medium,
        "Low" => odin_kernel::Severity::Low,
        _ => odin_kernel::Severity::Informational,
    }
}

fn parse_status(s: &str) -> odin_kernel::IncidentStatus {
    match s {
        "New" => odin_kernel::IncidentStatus::New,
        "Investigating" => odin_kernel::IncidentStatus::Investigating,
        "Contained" => odin_kernel::IncidentStatus::Contained,
        "Eradicated" => odin_kernel::IncidentStatus::Eradicated,
        "Recovered" => odin_kernel::IncidentStatus::Recovered,
        "Closed" => odin_kernel::IncidentStatus::Closed,
        _ => odin_kernel::IncidentStatus::New,
    }
}

fn parse_knowledge_status(s: &str) -> KnowledgeStatus {
    match s {
        "Draft" => KnowledgeStatus::Draft,
        "Review" => KnowledgeStatus::Review,
        "Active" => KnowledgeStatus::Active,
        "Deprecated" => KnowledgeStatus::Deprecated,
        "Archived" => KnowledgeStatus::Archived,
        "Purged" => KnowledgeStatus::Purged,
        _ => KnowledgeStatus::Draft,
    }
}

fn parse_knowledge_type(s: &str) -> KnowledgeType {
    match s {
        "Playbook" => KnowledgeType::Playbook,
        "ThreatIntel" => KnowledgeType::ThreatIntel,
        "MitreMapping" => KnowledgeType::MitreMapping,
        "IocDefinition" => KnowledgeType::IocDefinition,
        "ResponseProcedure" => KnowledgeType::ResponseProcedure,
        "Policy" => KnowledgeType::Policy,
        other => KnowledgeType::Custom(other.to_string()),
    }
}
