import { Box } from "@radix-ui/themes";
import { TbSquare } from "react-icons/tb";
import { BaseIcon } from "./BaseIcon";

type MaintainedProps = {
  n: number;
};
export const Threshold = ({ n }: MaintainedProps) => {
  return (
    <Box
      style={{
        verticalAlign: "text-top",
        display: "block",
        position: "relative",
      }}
    >
      <BaseIcon size={26} inline>
        <TbSquare />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          left: "50%",
          top: "45%",
          zIndex: 1,
          color: "black",
          transform: "translate(-50%, -50%)",
          fontSize: `calc(12px * var(--scaling))`,
          fontWeight: "bold",
        }}
      >
        {n}
      </Box>
    </Box>
  );
};
