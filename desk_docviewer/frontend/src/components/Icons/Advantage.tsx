import { Box } from "@radix-ui/themes";
import { BsFillDiamondFill } from "react-icons/bs";
import { BaseIcon } from "./BaseIcon";

type AdvantageProps = {
  n: number;
};

export const Advantage = ({ n }: AdvantageProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        position: "relative",
        verticalAlign: "middle",
      }}
    >
      <BaseIcon size={17}>
        <BsFillDiamondFill />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          fontWeight: "bold",
          color: "white",
          top: "50%",
          left: "50%",
          transform: "translate(-50%, -50%)",
          fontSize: "calc(10px * var(--scaling))",
        }}
      >
        {n >= 0 ? `+${n}` : n}
      </Box>
    </Box>
  );
};
