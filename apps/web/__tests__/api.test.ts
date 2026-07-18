import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockApi } from "../lib/api";

describe("mockApi", () => {
  it("health returns ok", async () => {
    const result = await mockApi.health();
    expect(result.status).toBe("ok");
    expect(result.version).toBe("0.1.0");
  });

  it("getIncident returns incident by id", async () => {
    const result = await mockApi.getIncident("inc-001");
    expect(result.id).toBe("inc-001");
    expect(result.title).toBeTruthy();
  });

  it("getGlobalGraph returns graph data", async () => {
    const result = await mockApi.getGlobalGraph();
    expect(result).toHaveProperty("nodes");
    expect(result).toHaveProperty("edges");
    expect(Array.isArray(result.nodes)).toBe(true);
  });

  it("searchSimilar returns results", async () => {
    const result = await mockApi.searchSimilar("inc-001", 3);
    expect(result.results).toBeDefined();
    expect(result.results.length).toBeLessThanOrEqual(3);
  });

  it("uploadIncident returns summary", async () => {
    const result = await mockApi.uploadIncident({
      title: "Test",
      description: "Test incident",
      severity: "high",
      evidence: [{ source: "test", content: "test", content_type: "log" }],
    });
    expect(result.id).toBeTruthy();
    expect(result.title).toBe("Test");
  });

  it("getConsolidationStats returns stats", async () => {
    const result = await mockApi.getConsolidationStats();
    expect(result.total_memories).toBe(11);
  });

  it("postFeedback resolves", async () => {
    const result = await mockApi.postFeedback("inc-001", "good", 5);
    expect(result).toBe("Feedback recorded");
  });

  it("updateStatus resolves", async () => {
    const result = await mockApi.updateStatus("inc-001", "Investigating");
    expect(result.new_status).toBe("Investigating");
  });
});
