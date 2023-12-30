import { describe, it, expect } from "vitest";
import { Interpreter } from "../src";

describe("base", () => {
  it("should pass", async () => {
    let interpreter = new Interpreter({
      read: (str: string) => str,
    });

    let result = interpreter.test("oooo");
    expect(result).toBe("oooo");
  });
});
