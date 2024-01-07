import { Box } from "@radix-ui/themes";
import { PiHourglassSimpleLowFill } from "react-icons/pi";
import { BaseIcon } from "./BaseIcon";
import { FaHourglass } from "react-icons/fa";

type MaintainedProps = {};
export const Maintained = ({}: MaintainedProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      <BaseIcon size={18} inline>
        <FaHourglass
          style={{
            color: "var(--gray-10)",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
