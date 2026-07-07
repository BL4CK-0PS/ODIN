# Entity Model

## Entity Types

```rust
enum EntityType {
    IpAddress,
    Domain,
    Url,
    Hash(Md5 | Sha1 | Sha256),
    EmailAddress,
    Hostname,
    UserName,
    Process,
    FilePath,
    RegistryKey,
    Mutex,
    ServiceName,
    Technique(TechniqueId),  // MITRE ATT&CK
    Campaign,
    ThreatActor,
}
```

## Entity Schema

```rust
struct Entity {
    id: Uuid,
    entity_type: EntityType,
    value: String,           // The actual value (e.g., "1.2.3.4")
    normalized_value: String, // Lowercased, trimmed
    metadata: HashMap<String, Value>,
    first_seen: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    workspace_id: Uuid,
}
```

## Entity Normalization

- **IP**: canonical string representation
- **Domain**: lowercase, punycode decoded
- **Hash**: lowercase, no colons
- **URL**: normalized scheme+host+path
- **Email**: lowercase
- **Hostname**: lowercase, FQDN format
