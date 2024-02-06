import { assert, describe, it } from "vitest";
import { normalizeDateTime } from "./normalizeDateTime";

describe("normalizeDateTime", () => {
  it("2022-01-01 13:20", () => {
    const str = "2022-01-01 13:20";
    assert.equal(normalizeDateTime(str), "2022-01-01T13:20");
  });

  it("2022-1-1 13:20", () => {
    const str = "2022-1-1 13";
    assert.equal(normalizeDateTime(str), "2022-01-01T13:00");
  });
});
