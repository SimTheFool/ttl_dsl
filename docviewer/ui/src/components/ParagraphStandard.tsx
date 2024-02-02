import { Theme, Text } from "@radix-ui/themes";

type ParagraphStandardProps = { children?: React.ReactNode };
export const ParagraphStandard = ({ children }: ParagraphStandardProps) => {
  return (
    <Text
      as="span"
      style={{
        display: "block",
        lineHeight: 1,
        fontStyle: "italic",
        fontSize: 14,
      }}
    >
      {children}
    </Text>
  );
};
