import { Box } from "@radix-ui/themes";
import { MdIncompleteCircle } from "react-icons/md";
import { BaseIcon } from "./BaseIcon";

type MaintainedProps = {
  n: string;
};
export const Duration = ({ n }: MaintainedProps) => {
  return (
    <Box
      style={{
        verticalAlign: "text-top",
        display: "block",
        position: "relative",
      }}
    >
      <BaseIcon size={26} inline>
        <MdIncompleteCircle
          style={{
            color: "var(--gray-10)",
          }}
        />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          left: "50%",
          top: "45%",
          zIndex: 1,
          color: "white",
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
