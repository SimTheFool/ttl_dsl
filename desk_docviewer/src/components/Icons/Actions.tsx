import { FaSquare } from "react-icons/fa";
import { IoTriangleSharp } from "react-icons/io5";
import { BaseIcon } from "./BaseIcon";
import { FaSquareFull } from "react-icons/fa";
import { Box } from "@radix-ui/themes";

type ActionProps = { size?: number };

export const MajorAction = ({ size }: ActionProps) => {
  return (
    <BaseIcon size={size || 18}>
      <FaSquareFull
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorAction = ({ size }: ActionProps) => {
  return (
    <BaseIcon size={size || 18}>
      <IoTriangleSharp
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorActionLight = ({ size }: ActionProps) => {
  return (
    <BaseIcon size={size || 18}>
      <IoTriangleSharp
        style={{
          color: "var(--gray-6)",
        }}
      />
    </BaseIcon>
  );
};

export const InlineMajorAction = ({ size }: ActionProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        verticalAlign: "text-top",
      }}
    >
      <BaseIcon size={size || 18}>
        <FaSquareFull
          style={{
            color: "var(--gray-11)",
          }}
        />
      </BaseIcon>
    </Box>
  );
};

export const InlineMinorAction = ({ size }: ActionProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        verticalAlign: "text-top",
      }}
    >
      <BaseIcon size={size || 18}>
        <IoTriangleSharp
          style={{
            color: "var(--gray-11)",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
