import { Box } from "@radix-ui/themes";
import { BaseIcon } from "./BaseIcon";
import { LiaDiceD6Solid } from "react-icons/lia";

type DiceSixProps = {};
export const DiceSix = ({}: DiceSixProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        verticalAlign: "text-top",
      }}
    >
      <BaseIcon size={14}>
        <LiaDiceD6Solid />
      </BaseIcon>
    </Box>
  );
};
