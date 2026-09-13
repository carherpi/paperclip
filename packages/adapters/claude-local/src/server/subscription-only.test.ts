import { describe, expect, it } from "vitest";
import { claudeSubscriptionOnlyViolation } from "./execute.js";

describe("Claude subscription-only mode", () => {
  it("rejects a configured API key", () => {
    expect(claudeSubscriptionOnlyViolation(
      { env: { ANTHROPIC_API_KEY: "sk-test" } },
      { PAPERCLIP_SUBSCRIPTION_ONLY: "1" },
    )).toContain("subscription authentication");
  });

  it("permits official CLI authentication", () => {
    expect(claudeSubscriptionOnlyViolation({}, { PAPERCLIP_SUBSCRIPTION_ONLY: "1" })).toBeNull();
  });
});
