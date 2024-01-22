import { Box } from "@radix-ui/themes";
import { TfiArrowsHorizontal } from "react-icons/tfi";
import { BaseIcon } from "./BaseIcon";

type SpellZoneProps = {
  zone: boolean;
};
export const SpellZone = ({ zone }: SpellZoneProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      {zone && (
        <BaseIcon size={12} inline>
          <TfiArrowsHorizontal
            style={{
              color: "black",
            }}
          />
        </BaseIcon>
      )}
    </Box>
  );
};
