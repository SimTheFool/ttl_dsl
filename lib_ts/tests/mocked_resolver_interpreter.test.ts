import { describe, it, expect } from "vitest";
import { Interpreter } from "../src";

const INDEX = `
    {
        stats:
        {
            <? ./stats
                with con : 5
                with vol : 3 >
            <? ./magician
                with mag : 4
                with initiation : 1
                with trad : "vol" >
        }
    }`;

const STATS = `"
    {
        con: $con
        vol: $vol
        ["con"]
        resist_phy: 0
        ["vol"]
        resist_ment: 0
        hit: 8
        ["vol" "con"]
        heal: 0
    }
    
    @TRANSFORM FINAL_STATS
    > $.resist_phy += $.con
    > $.resist_ment += $.vol
    > $.hit += floor($.con / 2)
    > $.heal += $.con + $.vol
    `;

const MAGICIAN = `"
    {
        mag: $mag
        initiation: $initiation
        [$trad]
        resist_drain: 0
    }
    
    @TRANSFORM FINAL_STATS_END
    > $.resist_drain += floor($.$trad / 2)
    `;

/* describe("mocked resolver interpreter", () => {
  let mockedResolver = {
    read: (str: string) => {
      switch (str) {
        case "./stats":
          return STATS;
        case "./magician":
          return MAGICIAN;
        default:
          throw new Error("Unknown file");
      }
    },
  };

  let mockedConfig = {
    get_transform_layers: () => ["FINAL_STATS", "FINAL_STATS_END"],
  };

  let formatter = new JSONFormatter();

  let interpreter = new Interpreter(mockedResolver, mockedConfig, formatter);

  it("should pass", async () => {
    let result = interpreter.assembleFromString(INDEX);

    expect(result.stats.con).toBe(5);
    expect(result.stats.con).toBe(3);
  });
});
 */
