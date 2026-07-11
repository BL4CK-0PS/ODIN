use neo4rs::*;
use odin_kernel::KernelError;
use serde::Serialize;

pub struct Neo4jClient {
    graph: Option<Graph>,
}

#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relationship: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl Neo4jClient {
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self, KernelError> {
        let config = ConfigBuilder::new()
            .uri(uri)
            .user(user)
            .password(password)
            .build()
            .map_err(|e| KernelError::Internal(format!("Neo4j config error: {}", e)))?;

        let graph = Graph::connect(config)
            .await
            .map_err(|e| KernelError::Internal(format!("Neo4j connection failed: {}", e)))?;

        tracing::info!("Connected to Neo4j at {}", uri);
        Ok(Self { graph: Some(graph) })
    }

    pub fn disconnected() -> Self {
        Self { graph: None }
    }

    pub fn is_connected(&self) -> bool {
        self.graph.is_some()
    }

    pub async fn health_check(&self) -> bool {
        if let Some(ref graph) = self.graph {
            graph.run(query("RETURN 1")).await.is_ok()
        } else {
            false
        }
    }

    pub async fn upsert_incident(
        &self,
        id: &str,
        title: &str,
        description: &str,
        severity: &str,
        status: &str,
        mitre_techniques: &[String],
    ) -> Result<(), KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let q = query(
            "MERGE (i:Incident {id: $id})
             SET i.title = $title,
                 i.description = $description,
                 i.severity = $severity,
                 i.status = $status,
                 i.mitre_techniques = $mitre_techniques,
                 i.updated_at = datetime()",
        )
        .param("id", id)
        .param("title", title)
        .param("description", description)
        .param("severity", severity)
        .param("status", status)
        .param("mitre_techniques", mitre_techniques.join(","));

        graph.run(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j upsert_incident failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn upsert_entity(
        &self,
        id: &str,
        name: &str,
        entity_type: &str,
    ) -> Result<(), KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let label = match entity_type {
            "IpAddress" => "IpAddress",
            "Domain" => "Domain",
            "Hash" => "Hash",
            "Process" => "Process",
            "File" => "File",
            "User" => "User",
            "Hostname" => "Hostname",
            _ => "Entity",
        };

        let q = query(&format!(
            "MERGE (e:{} {{id: $id}})
             SET e.name = $name,
                 e.entity_type = $entity_type,
                 e.updated_at = datetime()",
            label
        ))
        .param("id", id)
        .param("name", name)
        .param("entity_type", entity_type);

        graph.run(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j upsert_entity failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn upsert_evidence(
        &self,
        id: &str,
        incident_id: &str,
        source: &str,
        content_type: &str,
        trust_score: f64,
    ) -> Result<(), KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let q = query(
            "MERGE (ev:Evidence {id: $id})
             SET ev.source = $source,
                 ev.content_type = $content_type,
                 ev.trust_score = $trust_score,
                 ev.updated_at = datetime()
             WITH ev
             MATCH (i:Incident {id: $incident_id})
             MERGE (ev)-[:EVIDENCE_OF]->(i)",
        )
        .param("id", id)
        .param("incident_id", incident_id)
        .param("source", source)
        .param("content_type", content_type)
        .param("trust_score", trust_score);

        graph.run(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j upsert_evidence failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn link_entity_to_incident(
        &self,
        entity_id: &str,
        incident_id: &str,
        relationship: &str,
    ) -> Result<(), KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let q = query(&format!(
            "MATCH (e {{id: $entity_id}}), (i:Incident {{id: $incident_id}})
             MERGE (e)-[:{}]->(i)",
            relationship
        ))
        .param("entity_id", entity_id)
        .param("incident_id", incident_id);

        graph.run(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j link_entity failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn link_entity_to_entity(
        &self,
        source_id: &str,
        target_id: &str,
        relationship: &str,
    ) -> Result<(), KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let q = query(&format!(
            "MATCH (a {{id: $source_id}}), (b {{id: $target_id}})
             MERGE (a)-[:{}]->(b)",
            relationship
        ))
        .param("source_id", source_id)
        .param("target_id", target_id);

        graph.run(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j link_entity_to_entity failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn get_incident_graph(&self, incident_id: &str) -> Result<GraphData, KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let q = query(
            "MATCH (i:Incident {id: $id})
             OPTIONAL MATCH (ev:Evidence)-[:EVIDENCE_OF]->(i)
             OPTIONAL MATCH (e)-[:OBSERVED_IN|ASSOCIATED_WITH|RELATED_TO]->(i)
             WHERE e:IpAddress OR e:Domain OR e:Hash OR e:Process OR e:File OR e:User OR e:Hostname OR e:Entity
             RETURN i, ev, e",
        )
        .param("id", incident_id);

        let mut txn = graph.start_txn().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j transaction failed: {}", e))
        })?;

        let mut result = txn.execute(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j execute failed: {}", e))
        })?;

        while let Some(row) = result.next(&mut txn).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j row fetch failed: {}", e))
        })? {
            if let Ok(node) = row.get::<Node>("i") {
                let id = node.get::<String>("id").unwrap_or_default();
                let props = serde_json::json!({
                    "id": id,
                    "title": node.get::<String>("title").unwrap_or_default(),
                    "severity": node.get::<String>("severity").unwrap_or_default(),
                    "status": node.get::<String>("status").unwrap_or_default(),
                });
                nodes.push(GraphNode {
                    id: id.clone(),
                    label: "Incident".into(),
                    properties: props,
                });
            }

            if let Ok(node) = row.get::<Node>("ev") {
                let id = node.get::<String>("id").unwrap_or_default();
                let props = serde_json::json!({
                    "id": id,
                    "source": node.get::<String>("source").unwrap_or_default(),
                    "content_type": node.get::<String>("content_type").unwrap_or_default(),
                });
                nodes.push(GraphNode {
                    id: id.clone(),
                    label: "Evidence".into(),
                    properties: props,
                });
                edges.push(GraphEdge {
                    id: format!("eof-{}", id),
                    source: id,
                    target: incident_id.to_string(),
                    relationship: "EVIDENCE_OF".into(),
                    properties: serde_json::json!({}),
                });
            }

            if let Ok(node) = row.get::<Node>("e") {
                let id = node.get::<String>("id").unwrap_or_default();
                let name = node.get::<String>("name").unwrap_or_default();
                let entity_type = node.get::<String>("entity_type").unwrap_or_default();
                if !nodes.iter().any(|n| n.id == id) {
                    let props = serde_json::json!({
                        "id": id,
                        "name": name,
                        "entity_type": entity_type,
                    });
                    nodes.push(GraphNode {
                        id: id.clone(),
                        label: entity_type,
                        properties: props,
                    });
                    edges.push(GraphEdge {
                        id: format!("assoc-{}", id),
                        source: id,
                        target: incident_id.to_string(),
                        relationship: "ASSOCIATED_WITH".into(),
                        properties: serde_json::json!({}),
                    });
                }
            }
        }

        txn.commit().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j commit failed: {}", e))
        })?;

        Ok(GraphData { nodes, edges })
    }

    pub async fn get_global_graph(&self) -> Result<GraphData, KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let q = query(
            "MATCH (n)
             WHERE n:Incident OR n:Evidence OR n:IpAddress OR n:Domain OR n:Hash
                OR n:Process OR n:File OR n:User OR n:Hostname OR n:Entity
             OPTIONAL MATCH (n)-[r]->(m)
             WHERE m:Incident OR m:Evidence OR m:IpAddress OR m:Domain OR m:Hash
                OR m:Process OR m:File OR m:User OR m:Hostname OR m:Entity
             RETURN n, type(r) AS rel_type, m",
        );

        let mut txn = graph.start_txn().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j transaction failed: {}", e))
        })?;

        let mut result = txn.execute(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j execute failed: {}", e))
        })?;

        let mut seen_nodes = std::collections::HashSet::new();

        while let Some(row) = result.next(&mut txn).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j row fetch failed: {}", e))
        })? {
            if let Ok(node) = row.get::<Node>("n") {
                let id = node.get::<String>("id").unwrap_or_default();
                if !seen_nodes.contains(&id) {
                    seen_nodes.insert(id.clone());
                    let label = node.labels().first().map(|s| s.to_string()).unwrap_or_default();
                    let props = serde_json::json!({
                        "id": id,
                        "name": node.get::<String>("name").unwrap_or_default(),
                        "title": node.get::<String>("title").unwrap_or_default(),
                    });
                    nodes.push(GraphNode { id, label, properties: props });
                }
            }

            if let (Ok(rel_type), Ok(target)) = (
                row.get::<String>("rel_type"),
                row.get::<Node>("m"),
            ) {
                let source_id = row.get::<Node>("n")
                    .ok()
                    .and_then(|n| n.get::<String>("id").ok())
                    .unwrap_or_default();
                let target_id = target.get::<String>("id").unwrap_or_default();
                if !source_id.is_empty() && !target_id.is_empty() {
                    edges.push(GraphEdge {
                        id: format!("{}-{}-{}", source_id, rel_type, target_id),
                        source: source_id,
                        target: target_id,
                        relationship: rel_type,
                        properties: serde_json::json!({}),
                    });
                }
            }
        }

        txn.commit().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j commit failed: {}", e))
        })?;

        Ok(GraphData { nodes, edges })
    }

    pub async fn search_entities(
        &self,
        query_text: &str,
        limit: usize,
    ) -> Result<Vec<GraphNode>, KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let cypher = format!(
            "MATCH (e:IpAddress|Domain|Hash|Process|File|User|Hostname|Entity)
             WHERE e.name CONTAINS $query OR e.id CONTAINS $query
             RETURN e
             LIMIT {}",
            limit
        );

        let q = query(&cypher).param("query", query_text);

        let mut txn = graph.start_txn().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j transaction failed: {}", e))
        })?;

        let mut result = txn.execute(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j execute failed: {}", e))
        })?;

        let mut nodes = Vec::new();
        while let Some(row) = result.next(&mut txn).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j row fetch failed: {}", e))
        })? {
            if let Ok(node) = row.get::<Node>("e") {
                let id = node.get::<String>("id").unwrap_or_default();
                let label = node.labels().first().map(|s| s.to_string()).unwrap_or_default();
                let props = serde_json::json!({
                    "id": id,
                    "name": node.get::<String>("name").unwrap_or_default(),
                    "entity_type": node.get::<String>("entity_type").unwrap_or_default(),
                });
                nodes.push(GraphNode { id, label, properties: props });
            }
        }

        txn.commit().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j commit failed: {}", e))
        })?;

        Ok(nodes)
    }

    pub async fn get_entity_neighbors(
        &self,
        entity_id: &str,
        depth: usize,
    ) -> Result<GraphData, KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let cypher = format!(
            "MATCH path = (start {{id: $id}})-[*1..{}]-()
             UNWIND nodes(path) AS n
             UNWIND relationships(path) AS r
             RETURN DISTINCT n, type(r) AS rel_type, startNode(r) AS src, endNode(r) AS tgt",
            depth
        );

        let q = query(&cypher).param("id", entity_id);

        let mut txn = graph.start_txn().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j transaction failed: {}", e))
        })?;

        let mut result = txn.execute(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j execute failed: {}", e))
        })?;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut seen = std::collections::HashSet::new();

        while let Some(row) = result.next(&mut txn).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j row fetch failed: {}", e))
        })? {
            if let Ok(node) = row.get::<Node>("n") {
                let id = node.get::<String>("id").unwrap_or_default();
                if !seen.contains(&id) {
                    seen.insert(id.clone());
                    let label = node.labels().first().map(|s| s.to_string()).unwrap_or_default();
                    let props = serde_json::json!({
                        "id": id,
                        "name": node.get::<String>("name").unwrap_or_default(),
                        "title": node.get::<String>("title").unwrap_or_default(),
                    });
                    nodes.push(GraphNode { id, label, properties: props });
                }
            }

            if let (Ok(rel_type), Ok(src), Ok(tgt)) = (
                row.get::<String>("rel_type"),
                row.get::<Node>("src"),
                row.get::<Node>("tgt"),
            ) {
                let src_id = src.get::<String>("id").unwrap_or_default();
                let tgt_id = tgt.get::<String>("id").unwrap_or_default();
                let edge_id = format!("{}-{}-{}", src_id, rel_type, tgt_id);
                if !edges.iter().any(|edge: &GraphEdge| edge.id == edge_id) {
                    edges.push(GraphEdge {
                        id: edge_id,
                        source: src_id,
                        target: tgt_id,
                        relationship: rel_type,
                        properties: serde_json::json!({}),
                    });
                }
            }
        }

        txn.commit().await.map_err(|e| {
            KernelError::Internal(format!("Neo4j commit failed: {}", e))
        })?;

        Ok(GraphData { nodes, edges })
    }

    pub async fn delete_incident(&self, incident_id: &str) -> Result<(), KernelError> {
        let graph = self.graph.as_ref().ok_or_else(|| {
            KernelError::Internal("Neo4j not connected".into())
        })?;

        let q = query(
            "MATCH (i:Incident {id: $id})
             DETACH DELETE i",
        )
        .param("id", incident_id);

        graph.run(q).await.map_err(|e| {
            KernelError::Internal(format!("Neo4j delete_incident failed: {}", e))
        })?;

        Ok(())
    }
}
