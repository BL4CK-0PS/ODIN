# IOC Model

## Observable Types

```rust
enum ObservableType {
    IpV4,
    IpV6,
    Domain,
    Url,
    Email,
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Imphash,
    Authentihash,
    Ssddeep,
    Mutex,
    RegistryKey,
    ServiceName,
    PipeName,
}
}

## IOC Schema

```rust
struct Ioc {
    id: Uuid,
    observable_type: ObservableType,
    value: String,
    normalized_value: String,
    context: Option<String>,     // How the IOC was used
    first_seen: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    tags: Vec<String>,
    kill_chain_phase: Option<String>,  // Recon, Weaponization, Delivery, etc.
    mitre_technique: Option<TechniqueId>,
    source_reputation: Option<f64>,   // -1.0 (malicious) to 1.0 (benign)
    enrichment: Option<EnrichmentData>,
}

struct EnrichmentData {
    vt_lookup: Option<VtResult>,
    dns_lookup: Option<DnsResult>,
    whois: Option<WhoisResult>,
    last_enriched: DateTime<Utc>,
}
```

## IOC Lifecycle

1. **Extracted** — from evidence, alerts, or analyst input
2. **Normalized** — canonical form for deduplication
3. **Enriched** — external lookups
4. **Correlated** — matched against knowledge graph
5. **Scored** — reputation calculation
6. **Shared** — optionally exported to MISP/ThreatConnect
