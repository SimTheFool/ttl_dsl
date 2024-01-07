import { Box } from "@radix-ui/themes";
import { CgDice6 } from "react-icons/cg";
import { BaseIcon } from "./BaseIcon";
import { GiCheckMark } from "react-icons/gi";

type SuccessProps = {};
export const Success = ({}: SuccessProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "middle",
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
      {/* <Box
        style={{
          position: "absolute",
          top: "37%",
          left: "47%",
          transform: "translate(-50%, -50%)",
        }}
      >
        <BaseIcon size={20} inline>
          <FaCheck />
        </BaseIcon>
      </Box> */}
    </Box>
  );
};
