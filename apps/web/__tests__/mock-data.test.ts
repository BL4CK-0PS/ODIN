import { describe, it, expect } from "vitest";
import { mockIncidents, getMockIncident } from "../lib/mock-data";

describe("mock-data", () => {
  it("exports at least 11 mock incidents", () => {
    expect(mockIncidents.length).toBeGreaterThanOrEqual(11);
  });

  it("each mock incident has required fields", () => {
    for (const inc of mockIncidents) {
      expect(inc.id).toBeTruthy();
      expect(inc.title).toBeTruthy();
      expect(inc.description).toBeTruthy();
      expect(inc.severity).toBeTruthy();
      expect(inc.status).toBeTruthy();
      expect(Array.isArray(inc.tags)).toBe(true);
      expect(Array.isArray(inc.evidence_ids)).toBe(true);
      expect(Array.isArray(inc.entity_ids)).toBe(true);
    }
  });

  it("getMockIncident returns an incident by id", () => {
    const first = mockIncidents[0];
    const result = getMockIncident(first.id);
    expect(result.id).toBe(first.id);
    expect(result.title).toBe(first.title);
  });

  it("getMockIncident returns first incident for unknown id", () => {
    const result = getMockIncident("nonexistent");
    expect(result).toBeDefined();
    expect(result.id).toBeTruthy();
  });
});
