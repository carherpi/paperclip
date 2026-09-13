import { describe, expect, it } from "vitest";
import { codexSubscriptionOnlyViolation } from "./execute.js";

describe("Codex subscription-only mode", () => {
  it("rejects a configured API key", () => {
    expect(codexSubscriptionOnlyViolation(
      { env: { OPENAI_API_KEY: "sk-test" } },
      { PAPERCLIP_SUBSCRIPTION_ONLY: "1" },
    )).toContain("subscription authentication");
  });

  it("permits official CLI authentication", () => {
    expect(codexSubscriptionOnlyViolation({}, { PAPERCLIP_SUBSCRIPTION_ONLY: "1" })).toBeNull();
  });
});
