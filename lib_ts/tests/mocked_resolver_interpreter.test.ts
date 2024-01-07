import { describe, it, expect } from "vitest";
import { InterpreterBuilder } from "../src";

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

const STATS = `
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

const MAGICIAN = `
    {
        mag: $mag
        initiation: $initiation
        [$trad]
        resist_drain: 0
    }
    
    @TRANSFORM FINAL_STATS_END
    > $.resist_drain += floor($.$trad / 2)
    `;

describe("mocked resolver interpreter", () => {
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
    getTransformLayers: () => ["FINAL_STATS", "FINAL_STATS_END"],
  };

  it("should pass", async () => {
    let interpreter = new InterpreterBuilder()
      .with_custom_resolver(mockedResolver)
      .with_custom_config_provider(mockedConfig)
      .with_json_formatter()
      .with_console_logger()
      .build();
    let result = interpreter.assemble_from_str(INDEX);

    expect(result.stats.con.value).toBe(5);
    expect(result.stats.vol.value).toBe(3);
    expect(result.stats.resist_phy.metas.includes("con")).toBe(true);
    expect(result.stats.resist_phy.value).toBe(5);
    expect(result.stats.resist_ment.metas.includes("vol")).toBe(true);
    expect(result.stats.resist_ment.value).toBe(3);
    expect(result.stats.hit.value).toBe(10);
    expect(result.stats.heal.value).toBe(8);
    expect(result.stats.resist_drain.metas.includes("vol")).toBe(true);
    expect(result.stats.resist_drain.value).toBe(1);
    expect(result.stats.mag.value).toBe(4);
    expect(result.stats.initiation.value).toBe(1);
  });
});
