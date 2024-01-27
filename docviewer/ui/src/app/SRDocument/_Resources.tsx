import {
  MajorAction,
  MinorAction,
  MinorActionLight,
} from "@/components/Icons/Actions";
import { Edge } from "@/components/Icons/Edge";
import { TitleMin } from "@/components/TitleMin";
import { Box, Flex } from "@radix-ui/themes";
import { SRCharacter } from "./character";

type ResourcesProps = {
  char: SRCharacter;
};

export const Resources = ({ char }: ResourcesProps) => {
  const minActionNb = Math.max(char.stats.action_min, 5);

  return (
    <Flex>
      <Box pr={"4"}>
        <TitleMin title={"Actions"} />
        <Box pr={"2"} display={"inline-block"}></Box>
        {Array.from({ length: char.stats.action_maj }).map((_, i) => (
          <Box pr={"1"} display={"inline-block"} key={i}>
            <MajorAction size={22} />
          </Box>
        ))}
        {Array.from({ length: minActionNb }).map((_, i) => (
          <Box pr={"2"} display={"inline-block"} key={i}>
            {i < char.stats.action_min ? (
              <MinorAction size={18} />
            ) : (
              <MinorActionLight size={18} />
            )}
          </Box>
        ))}
      </Box>
      <Box>
        <TitleMin title={"Atouts"} />
        {Array.from({ length: char.stats.edge }).map((_, i) => (
          <Box pr={"2"} display={"inline-block"} key={i}>
            <Edge />
          </Box>
        ))}
      </Box>
    </Flex>
  );
};
