#!/bin/bash
# =============================================================================
# Neo4j Initialization Script
# Creates constraints and indexes for the ODIN knowledge graph.
# Usage: ./init.sh (run from host after Neo4j is up)
# =============================================================================
# This script should be run once after the first Neo4j startup to create
# performance-optimizing constraints and indexes.
# =============================================================================

set -euo pipefail

NEO4J_CONTAINER="${NEO4J_CONTAINER:-odin-neo4j-1}"
NEO4J_USER="${NEO4J_USER:-neo4j}"
NEO4J_PASSWORD="${NEO4J_PASSWORD:-odin}"

echo "[+] Waiting for Neo4j container to be ready..."
until docker exec "$NEO4J_CONTAINER" cypher-shell -u "$NEO4J_USER" -p "$NEO4J_PASSWORD" "RETURN 1" 2>/dev/null; do
    sleep 2
done
echo "[+] Neo4j is ready. Creating constraints and indexes..."

CYPHER_QUERIES="
CREATE CONSTRAINT incident_id_unique IF NOT EXISTS FOR (i:Incident) REQUIRE i.id IS UNIQUE;
CREATE CONSTRAINT evidence_id_unique IF NOT EXISTS FOR (ev:Evidence) REQUIRE ev.id IS UNIQUE;
CREATE CONSTRAINT ip_id_unique IF NOT EXISTS FOR (ip:IpAddress) REQUIRE ip.id IS UNIQUE;
CREATE CONSTRAINT domain_id_unique IF NOT EXISTS FOR (d:Domain) REQUIRE d.id IS UNIQUE;
CREATE CONSTRAINT hash_id_unique IF NOT EXISTS FOR (h:Hash) REQUIRE h.id IS UNIQUE;
CREATE CONSTRAINT process_id_unique IF NOT EXISTS FOR (p:Process) REQUIRE p.id IS UNIQUE;
CREATE CONSTRAINT file_id_unique IF NOT EXISTS FOR (f:File) REQUIRE f.id IS UNIQUE;
CREATE CONSTRAINT hostname_id_unique IF NOT EXISTS FOR (h:Hostname) REQUIRE h.id IS UNIQUE;
CREATE CONSTRAINT entity_id_unique IF NOT EXISTS FOR (e:Entity) REQUIRE e.id IS UNIQUE;
CREATE INDEX incident_severity IF NOT EXISTS FOR (i:Incident) ON (i.severity);
CREATE INDEX incident_status IF NOT EXISTS FOR (i:Incident) ON (i.status);
CREATE INDEX entity_name IF NOT EXISTS FOR (e:Entity) ON (e.name);
CREATE INDEX entity_type IF NOT EXISTS FOR (e:Entity) ON (e.entity_type);
"

echo "$CYPHER_QUERIES" | docker exec -i "$NEO4J_CONTAINER" cypher-shell -u "$NEO4J_USER" -p "$NEO4J_PASSWORD"

echo "[+] Neo4j initialization complete."
