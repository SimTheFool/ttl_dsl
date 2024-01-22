import { Box } from "@radix-ui/themes";
import { PiEyeLight, PiHandWaving } from "react-icons/pi";
import { BaseIcon } from "./BaseIcon";
import { IoIosBody } from "react-icons/io";

type SpellDistanceProps = {
  range: "contact" | "LDV" | "perso";
};
export const SpellDistance = ({ range }: SpellDistanceProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      {range == "contact" && (
        <BaseIcon size={12} inline>
          <PiHandWaving
            style={{
              color: "black",
            }}
          />
        </BaseIcon>
      )}

      {range == "LDV" && (
        <BaseIcon size={12} inline>
          <PiEyeLight
            style={{
              color: "black",
            }}
          />
        </BaseIcon>
      )}

      {range == "perso" && (
        <BaseIcon size={12} inline>
          <IoIosBody />
        </BaseIcon>
      )}
    </Box>
  );
};
