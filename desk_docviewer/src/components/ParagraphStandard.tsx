import { Theme, Text } from "@radix-ui/themes";

type ParagraphStandardProps = { children?: React.ReactNode };
export const ParagraphStandard = ({ children }: ParagraphStandardProps) => {
  return (
    <Text
      size={"2"}
      as="span"
      style={{ display: "block", lineHeight: 1, fontStyle: "italic" }}
    >
      {children}
    </Text>
  );
};
