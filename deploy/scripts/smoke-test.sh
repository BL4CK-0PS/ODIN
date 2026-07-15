#!/bin/bash
# =============================================================================
# ODIN Post-Deploy Smoke Test
# Validates that all services are up and responding.
# Usage: ./smoke-test.sh [base_url]
# =============================================================================

set -euo pipefail

BASE_URL="${1:-http://localhost}"
API_URL="${BASE_URL}:3001"
PASS=0
FAIL=0

check() {
    local desc="$1"
    local url="$2"
    local expected_status="${3:-200}"

    STATUS=$(curl -s -o /dev/null -w "%{http_code}" --max-time 10 "$url" 2>/dev/null || echo "000")
    if [ "$STATUS" = "$expected_status" ]; then
        echo "  [PASS] $desc (HTTP $STATUS)"
        PASS=$((PASS + 1))
    else
        echo "  [FAIL] $desc (expected HTTP $expected_status, got HTTP $STATUS)"
        FAIL=$((FAIL + 1))
    fi
}

echo "============================================"
echo " ODIN Smoke Test"
echo " Target: $BASE_URL"
echo "============================================"
echo ""

echo "[1/4] Core Services"
check "Nginx"           "$BASE_URL"                200
check "API Health"      "$API_URL/api/v1/system/health" 200
check "API Version"     "$API_URL/api/v1/system/version" 200
check "Metrics Endpoint" "$API_URL/metrics"        200
echo ""

echo "[2/4] Authentication"
LOGIN_RESP=$(curl -s -X POST "$API_URL/api/v1/auth/login" \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"odin-dev-password"}' 2>/dev/null || echo "{}")

TOKEN=$(echo "$LOGIN_RESP" | grep -o '"token":"[^"]*"' | head -1 | cut -d'"' -f4)
if [ -n "$TOKEN" ]; then
    echo "  [PASS] Login successful, token obtained"
    PASS=$((PASS + 1))
else
    echo "  [FAIL] Login failed"
    FAIL=$((FAIL + 1))
    TOKEN=""
fi
echo ""

echo "[3/4] Protected Endpoints (with auth)"
AUTH_HEADER=""
if [ -n "$TOKEN" ]; then
    AUTH_HEADER="Authorization: Bearer $TOKEN"
    check "System Stats"     "$API_URL/api/v1/system/stats"         200
    check "List Incidents"   "$API_URL/api/v1/incidents"            200
    check "List Memories"    "$API_URL/api/v1/memories"             200
    check "Knowledge List"   "$API_URL/api/v1/knowledge/list"       200
    check "Global Graph"     "$API_URL/api/v1/graph"                200
    check "Audit Logs"       "$API_URL/api/v1/auth/audit"           200
    check "Consolidation"    "$API_URL/api/v1/consolidation/stats"  200
else
    echo "  [SKIP] No auth token, skipping protected endpoint tests"
fi
echo ""

echo "[4/4] Infrastructure"
check "Qdrant"    "http://localhost:6333/healthz"   200
check "Redis"     "http://localhost:6379"            200
echo ""

echo "============================================"
echo " Results: $PASS passed, $FAIL failed"
echo "============================================"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
exit 0
