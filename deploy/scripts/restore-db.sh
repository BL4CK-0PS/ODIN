#!/bin/bash
# =============================================================================
# ODIN Database Restore Script
# Restores a PostgreSQL backup created by backup-db.sh.
# Usage: ./restore-db.sh <backup_file.sql.gz>
# =============================================================================

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <backup_file.sql.gz>"
    echo "Example: $0 ./backups/odin_backup_20260715_120000.sql.gz"
    exit 1
fi

BACKUP_FILE="$1"
CONTAINER_NAME="odin-postgres-1"
DB_NAME="${POSTGRES_DB:-odin}"
DB_USER="${POSTGRES_USER:-odin}"

if [ ! -f "$BACKUP_FILE" ]; then
    echo "[-] Error: Backup file not found: $BACKUP_FILE"
    exit 1
fi

echo "[!] WARNING: This will overwrite the current database '$DB_NAME'."
echo "    Backup file: $BACKUP_FILE"
read -p "    Continue? (yes/no): " CONFIRM
if [ "$CONFIRM" != "yes" ]; then
    echo "Aborted."
    exit 0
fi

echo "[+] Dropping and recreating database..."
docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -d postgres -c "DROP DATABASE IF EXISTS $DB_NAME;"
docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -d postgres -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;"

echo "[+] Restoring from backup..."
gunzip -c "$BACKUP_FILE" | docker exec -i "$CONTAINER_NAME" psql -U "$DB_USER" -d "$DB_NAME" --quiet

echo "[+] Restore complete. Verifying tables..."
docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -d "$DB_NAME" -c "\dt"

echo "[+] Done."
