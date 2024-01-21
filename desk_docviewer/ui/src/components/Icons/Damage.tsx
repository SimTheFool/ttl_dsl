import { Box } from "@radix-ui/themes";
import { GiGooeyImpact } from "react-icons/gi";
import { BaseIcon } from "./BaseIcon";

type DamageProps = {};
export const Damage = ({}: DamageProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-bottom",
      }}
    >
      <BaseIcon size={12} inline>
        <GiGooeyImpact
          style={{
            color: "black",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
