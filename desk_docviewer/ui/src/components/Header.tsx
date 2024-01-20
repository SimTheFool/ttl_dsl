import { SRCharacter } from "@/app/SRDocument/character";
import { Box, Flex, Heading, Text } from "@radix-ui/themes";
import { ReactNode } from "react";

type HeaderProps = {
  char: SRCharacter;
};

export const Header = ({ char }: HeaderProps) => {
  return (
    <Flex asChild justify={"center"}>
      <Heading>
        <HeadingBlock>
          <Text
            style={{
              textTransform: "uppercase",
            }}
            align={"right"}
          >
            {char.name}
          </Text>
        </HeadingBlock>

        <HeadingBlock>
          <Text
            size={"2"}
            align={"left"}
            style={{
              alignSelf: "flex-end",
              lineHeight: "1.65",
            }}
          >
            {Object.values(char.tags).join(" - ")}
          </Text>
        </HeadingBlock>
      </Heading>
    </Flex>
  );
};

const HeadingBlock = ({ children }: { children: ReactNode }) => {
  return (
    <Box
      style={{
        maxWidth: "50%",
      }}
      display={"inline-block"}
      asChild
      grow={"1"}
      p={"1"}
    >
      {children}
    </Box>
  );
};
