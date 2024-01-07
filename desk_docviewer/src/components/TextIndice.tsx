import { Theme, Text } from "@radix-ui/themes";

type TextIndiceProps = { children?: React.ReactNode };
export const TextIndice = ({ children }: TextIndiceProps) => {
  return (
    <Text
      as="span"
      style={{
        display: "inline",
        lineHeight: 1,
        fontSize: `calc(11px * var(--scaling))`,
        color: "var(--gray-10)",
      }}
    >
      {children}
    </Text>
  );
};
