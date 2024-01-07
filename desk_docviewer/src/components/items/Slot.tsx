import { Box, Text } from "@radix-ui/themes";
import { Space } from "../Space";
import { TextIndice } from "../TextIndice";

type SlotProps = {
  note?: React.ReactNode;
  size: "S" | "M" | "L" | "XL" | "XXL" | "INF";
  concealment?: number;
  children?: React.ReactNode;
};

const sizes = {
  S: "25px",
  M: "35px",
  L: "50px",
  XL: "120px",
  XXL: "250px",
  INF: "100%",
} satisfies Record<SlotProps["size"], string>;

export const Slot = ({ children, note, size, concealment }: SlotProps) => {
  return (
    <Box
      p={"2"}
      style={{
        border: "1px dashed var(--gray-10)",
        height: sizes[size],
        position: "relative",
      }}
    >
      <Box
        style={{
          width: "100%",
          height: "100%",
          overflow: "hidden",
        }}
      >
        {children}
      </Box>
      <Text
        size={"1"}
        style={{
          zIndex: 1,
          position: "absolute",
          bottom: "0",
          right: "0",
          lineHeight: "0.5",
          transform: "translate(0%, 2px)",
          backgroundColor: "white",
          paddingLeft: "var(--space-1)",
          paddingRight: "var(--space-1)",
        }}
      >
        {note}
        {concealment && (
          <>
            <Space inline />
            <TextIndice>{concealment && `(d${concealment})`}</TextIndice>
          </>
        )}
      </Text>
    </Box>
  );
};
