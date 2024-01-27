import { Box, Flex } from "@radix-ui/themes";
import React from "react";
import { ParagraphStandard } from "./ParagraphStandard";

type GaugeProps = {
  length: number;
  icon?: React.ReactNode;
};

const STEP = 5;
const SWITCH = 30;

export const Gauge = ({ length, icon }: GaugeProps) => {
  const group = Math.floor(length / STEP);
  const remainder = length % STEP;

  return (
    <>
      {length <= SWITCH && (
        <Flex
          wrap={"wrap"}
          style={{
            lineHeight: "0px",
          }}
        >
          {Array.from({ length: group }, (_, i) => (
            <Box pr={"1"} key={i}>
              {Array.from({ length: STEP }, (_, j) => (
                <Box
                  key={[j, i].join(" ")}
                  style={{
                    display: "inline-block",
                  }}
                >
                  {icon}
                </Box>
              ))}
            </Box>
          ))}
          {remainder > 0 &&
            Array.from({ length: remainder }, (_, i) => (
              <Box
                key={[i].join(" ")}
                style={{
                  display: "inline-block",
                }}
              >
                {icon}
              </Box>
            ))}
        </Flex>
      )}
      {length > SWITCH && (
        <Flex>
          <Box>-------------------</Box>
          <ParagraphStandard>
            /{length}
            {icon}
          </ParagraphStandard>
        </Flex>
      )}
    </>
  );
};
