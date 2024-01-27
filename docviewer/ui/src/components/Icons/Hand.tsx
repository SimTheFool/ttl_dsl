import { Box } from "@radix-ui/themes";
import { IoHandLeft } from "react-icons/io5";
import { BaseIcon } from "./BaseIcon";

type HandProps = {
  n?: number;
};

export const Hand = ({ n }: HandProps) => {
  return (
    <Box
      style={{
        display: "block",
        position: "relative",
      }}
    >
      <BaseIcon size={24}>
        <IoHandLeft
          style={{
            color: "var(--gray-10)",
          }}
        />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          left: "40%",
          top: "70%",
          zIndex: 1,
          color: "white",
          transform: "translate(-50%, -50%)",
          fontSize: `calc(10px * var(--scaling))`,
          fontWeight: "bold",
        }}
      >
        {n}
      </Box>
    </Box>
  );
};
