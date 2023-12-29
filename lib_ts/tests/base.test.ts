import { describe, it, expect } from "vitest";
import { getInterpreter } from "../src";

describe("base", () => {
  it("should pass", async () => {
    let interpreter = await getInterpreter();
    let result = interpreter.test("oooo");
    expect(result).toBe("oooo");
  });
});
