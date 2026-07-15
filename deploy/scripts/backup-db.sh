#!/bin/bash
# =============================================================================
# ODIN Database Backup Script
# Creates timestamped PostgreSQL backups with optional compression.
# Usage: ./backup-db.sh [output_directory]
# =============================================================================

set -euo pipefail

OUTPUT_DIR="${1:-./backups}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="odin_backup_${TIMESTAMP}"
CONTAINER_NAME="odin-postgres-1"
DB_NAME="${POSTGRES_DB:-odin}"
DB_USER="${POSTGRES_USER:-odin}"

mkdir -p "$OUTPUT_DIR"

echo "[+] Starting ODIN database backup..."
echo "    Database: $DB_NAME"
echo "    Output:   $OUTPUT_DIR/${BACKUP_NAME}.sql.gz"

docker exec "$CONTAINER_NAME" pg_dump -U "$DB_USER" -d "$DB_NAME" --no-owner --no-acl \
    | gzip > "$OUTPUT_DIR/${BACKUP_NAME}.sql.gz"

FILESIZE=$(du -h "$OUTPUT_DIR/${BACKUP_NAME}.sql.gz" | cut -f1)
echo "[+] Backup complete: $OUTPUT_DIR/${BACKUP_NAME}.sql.gz ($FILESIZE)"

# Retain only the last 30 backups
BACKUP_COUNT=$(ls -1 "$OUTPUT_DIR"/odin_backup_*.sql.gz 2>/dev/null | wc -l)
if [ "$BACKUP_COUNT" -gt 30 ]; then
    REMOVED=$(ls -1t "$OUTPUT_DIR"/odin_backup_*.sql.gz | tail -n +31 | xargs rm -v)
    echo "[+] Cleaned old backups. Retained 30 most recent."
fi

echo "[+] Done."
