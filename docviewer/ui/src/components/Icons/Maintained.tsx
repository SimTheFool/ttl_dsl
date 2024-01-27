import { Box } from "@radix-ui/themes";
import { FaHourglass } from "react-icons/fa";
import { BaseIcon } from "./BaseIcon";

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
