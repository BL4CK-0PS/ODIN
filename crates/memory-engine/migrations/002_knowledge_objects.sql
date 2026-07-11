CREATE TABLE IF NOT EXISTS knowledge_objects (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT '',
    object_type TEXT NOT NULL DEFAULT 'Custom',
    status TEXT NOT NULL DEFAULT 'Draft',
    tags TEXT[] NOT NULL DEFAULT '{}',
    source_incidents TEXT[] NOT NULL DEFAULT '{}',
    mitre_techniques TEXT[] NOT NULL DEFAULT '{}',
    confidence_sources JSONB NOT NULL DEFAULT '[]',
    created_by TEXT NOT NULL DEFAULT 'system',
    updated_by TEXT NOT NULL DEFAULT 'system',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status_history JSONB NOT NULL DEFAULT '[]',
    expires_at TIMESTAMPTZ,
    review_notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_knowledge_objects_status ON knowledge_objects(status);
CREATE INDEX IF NOT EXISTS idx_knowledge_objects_type ON knowledge_objects(object_type);
CREATE INDEX IF NOT EXISTS idx_knowledge_objects_tags ON knowledge_objects USING GIN(tags);
