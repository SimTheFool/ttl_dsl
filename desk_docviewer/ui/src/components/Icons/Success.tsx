import { Box } from "@radix-ui/themes";
import { GiCheckMark } from "react-icons/gi";
import { BaseIcon } from "./BaseIcon";

type SuccessProps = {};
export const Success = ({}: SuccessProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-bottom",
        position: "relative",
      }}
    >
      <BaseIcon size={14} inline>
        <GiCheckMark
          style={{
            color: "black",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
