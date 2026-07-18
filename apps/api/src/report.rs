use odin_core::odin_kernel::{
    CanonicalIncident, Entity, EntityType, Evidence, IncidentStatus, Severity,
};

fn severity_color(s: &Severity) -> &'static str {
    match s {
        Severity::Critical => "#dc2626",
        Severity::High => "#ea580c",
        Severity::Medium => "#d97706",
        Severity::Low => "#2563eb",
        Severity::Informational => "#6b7280",
    }
}

fn status_color(s: &IncidentStatus) -> &'static str {
    match s {
        IncidentStatus::New => "#3b82f6",
        IncidentStatus::Investigating => "#f59e0b",
        IncidentStatus::Contained => "#f97316",
        IncidentStatus::Eradicated => "#ef4444",
        IncidentStatus::Recovered => "#22c55e",
        IncidentStatus::Closed => "#6b7280",
    }
}

fn format_severity(s: &Severity) -> &'static str {
    match s {
        Severity::Critical => "Critical",
        Severity::High => "High",
        Severity::Medium => "Medium",
        Severity::Low => "Low",
        Severity::Informational => "Informational",
    }
}

fn format_status(s: &IncidentStatus) -> &'static str {
    match s {
        IncidentStatus::New => "New",
        IncidentStatus::Investigating => "Investigating",
        IncidentStatus::Contained => "Contained",
        IncidentStatus::Eradicated => "Eradicated",
        IncidentStatus::Recovered => "Recovered",
        IncidentStatus::Closed => "Closed",
    }
}

fn entity_type_label(t: &EntityType) -> &'static str {
    match t {
        EntityType::IpAddress => "IP Address",
        EntityType::Domain => "Domain",
        EntityType::File => "File",
        EntityType::Process => "Process",
        EntityType::User => "User",
        EntityType::Hostname => "Hostname",
        EntityType::Hash => "Hash",
        EntityType::NetworkConnection => "Network Connection",
        EntityType::Artifact => "Artifact",
        EntityType::Other(_) => "Other",
    }
}

pub fn generate_html_report(
    incident: &CanonicalIncident,
    evidence: &[Evidence],
    entities: &[Entity],
    memory_summary: Option<&str>,
    narrative: Option<&str>,
    playbook_steps: &[String],
) -> String {
    let evidence_rows: String = evidence
        .iter()
        .enumerate()
        .map(|(i, ev)| {
            format!(
                r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td><code>{}</code></td>
                <td>{:.0}%</td>
                <td>{}</td>
            </tr>"#,
                i + 1,
                html_escape(&ev.source),
                html_escape(&format!("{:?}", ev.content_type)),
                ev.trust_score * 100.0,
                ev.collected_at.format("%Y-%m-%d %H:%M UTC"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let entity_rows: String = entities
        .iter()
        .enumerate()
        .map(|(i, ent)| {
            format!(
                r#"<tr>
                <td>{}</td>
                <td><span class="badge badge-type">{}</span></td>
                <td><code>{}</code></td>
            </tr>"#,
                i + 1,
                entity_type_label(&ent.entity_type),
                html_escape(&ent.name),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let tech_badges: String = incident
        .mitre_techniques
        .iter()
        .map(|t| {
            format!(
                r#"<span class="badge badge-tech">{}</span>"#,
                html_escape(t)
            )
        })
        .collect::<Vec<_>>()
        .join(" ");

    let tag_badges: String = incident
        .tags
        .iter()
        .map(|t| format!(r#"<span class="badge badge-tag">{}</span>"#, html_escape(t)))
        .collect::<Vec<_>>()
        .join(" ");

    let playbook_html: String = if playbook_steps.is_empty() {
        "<p class=\"muted\">No playbook generated.</p>".to_string()
    } else {
        let items: String = playbook_steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                format!(
                    "<li><strong>{}</strong> — {}</li>",
                    i + 1,
                    html_escape(step)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("<ol class=\"playbook-list\">{}</ol>", items)
    };

    let narrative_html = narrative.map(|n| {
        format!(r#"<div class="section"><h2>Attack Narrative</h2><p class="narrative">{}</p></div>"#, html_escape(n))
    }).unwrap_or_default();

    let memory_html = memory_summary
        .map(|m| {
            format!(
                r#"<div class="section"><h2>Threat Memory Summary</h2><p>{}</p></div>"#,
                html_escape(m)
            )
        })
        .unwrap_or_default();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Incident Report — {title}</title>
<style>
  :root {{
    --bg: #0f172a; --fg: #e2e8f0; --muted: #94a3b8; --border: #1e293b;
    --card: #1e293b; --accent: #38bdf8;
  }}
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{ font-family: 'Inter', -apple-system, sans-serif; background: var(--bg); color: var(--fg); padding: 40px; line-height: 1.6; }}
  .report {{ max-width: 900px; margin: 0 auto; }}
  .header {{ border-bottom: 2px solid var(--border); padding-bottom: 20px; margin-bottom: 30px; }}
  .header h1 {{ font-size: 28px; font-weight: 700; margin-bottom: 8px; }}
  .header .meta {{ display: flex; gap: 16px; flex-wrap: wrap; }}
  .badge {{ display: inline-block; padding: 2px 10px; border-radius: 6px; font-size: 12px; font-weight: 600; }}
  .badge-severity {{ color: white; background: {sev_color}; }}
  .badge-status {{ color: white; background: {status_color}; }}
  .badge-type {{ background: #334155; color: #94a3b8; }}
  .badge-tech {{ background: #1e3a5f; color: #7dd3fc; }}
  .badge-tag {{ background: #1e293b; color: #94a3b8; border: 1px solid #334155; }}
  .section {{ margin-bottom: 28px; }}
  .section h2 {{ font-size: 18px; font-weight: 600; margin-bottom: 12px; color: var(--accent); }}
  table {{ width: 100%; border-collapse: collapse; font-size: 13px; }}
  th, td {{ text-align: left; padding: 8px 12px; border-bottom: 1px solid var(--border); }}
  th {{ color: var(--muted); font-weight: 500; font-size: 11px; text-transform: uppercase; letter-spacing: 0.05em; }}
  code {{ background: #0f172a; padding: 2px 6px; border-radius: 4px; font-size: 12px; }}
  .narrative {{ background: var(--card); padding: 16px; border-radius: 8px; border-left: 3px solid var(--accent); }}
  .muted {{ color: var(--muted); }}
  .playbook-list {{ padding-left: 20px; }}
  .playbook-list li {{ margin-bottom: 8px; }}
  .footer {{ margin-top: 40px; padding-top: 20px; border-top: 1px solid var(--border); font-size: 11px; color: var(--muted); text-align: center; }}
  @media print {{
    body {{ background: white; color: #1e293b; padding: 20px; }}
    :root {{ --bg: white; --fg: #1e293b; --muted: #64748b; --border: #e2e8f0; --card: #f8fafc; --accent: #0284c7; }}
    .badge-severity {{ color: white; }}
    .badge-status {{ color: white; }}
    .badge-type {{ background: #f1f5f9; color: #475569; }}
    .badge-tech {{ background: #e0f2fe; color: #0369a1; }}
    .badge-tag {{ background: #f8fafc; color: #475569; border-color: #e2e8f0; }}
    code {{ background: #f1f5f9; }}
    .narrative {{ background: #f8fafc; }}
  }}
</style>
</head>
<body>
<div class="report">
  <div class="header">
    <h1>{title}</h1>
    <div class="meta">
      <span class="badge badge-severity">{severity}</span>
      <span class="badge badge-status">{status}</span>
      <span style="color: var(--muted); font-size: 13px;">ID: <code>{id}</code></span>
      <span style="color: var(--muted); font-size: 13px;">Created: {created}</span>
      <span style="color: var(--muted); font-size: 13px;">Updated: {updated}</span>
    </div>
  </div>

  <div class="section">
    <h2>Description</h2>
    <p>{description}</p>
  </div>

  <div class="section">
    <h2>MITRE ATT&CK Techniques</h2>
    <div>{techs}</div>
  </div>

  <div class="section">
    <h2>Tags</h2>
    <div>{tags}</div>
  </div>

  {narrative}

  {memory}

  <div class="section">
    <h2>Evidence ({evidence_count} items)</h2>
    <table>
      <thead>
        <tr><th>#</th><th>Source</th><th>Type</th><th>Trust</th><th>Collected</th></tr>
      </thead>
      <tbody>
        {evidence_rows}
      </tbody>
    </table>
  </div>

  <div class="section">
    <h2>Entities ({entity_count} items)</h2>
    <table>
      <thead>
        <tr><th>#</th><th>Type</th><th>Name</th></tr>
      </thead>
      <tbody>
        {entity_rows}
      </tbody>
    </table>
  </div>

  <div class="section">
    <h2>Response Playbook</h2>
    {playbook}
  </div>

  <div class="footer">
    Generated by ODIN — Operational Defense Intelligence Network · {generated_at}
  </div>
</div>
</body>
</html>"#,
        title = html_escape(&incident.title),
        id = html_escape(&incident.id),
        severity = format_severity(&incident.severity),
        status = format_status(&incident.status),
        created = incident.created_at.format("%Y-%m-%d %H:%M UTC"),
        updated = incident.updated_at.format("%Y-%m-%d %H:%M UTC"),
        description = html_escape(&incident.description),
        techs = if tech_badges.is_empty() {
            "<span class=\"muted\">None mapped</span>".to_string()
        } else {
            tech_badges
        },
        tags = if tag_badges.is_empty() {
            "<span class=\"muted\">None</span>".to_string()
        } else {
            tag_badges
        },
        narrative = narrative_html,
        memory = memory_html,
        evidence_count = evidence.len(),
        evidence_rows = evidence_rows,
        entity_count = entities.len(),
        entity_rows = entity_rows,
        playbook = playbook_html,
        generated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"),
        sev_color = severity_color(&incident.severity),
        status_color = status_color(&incident.status),
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
