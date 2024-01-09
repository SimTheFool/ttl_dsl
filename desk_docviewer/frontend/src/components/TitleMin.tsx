import { Box, Heading, Text } from "@radix-ui/themes";
import React from "react";

type MinTitleProps = {
  title?: React.ReactNode;
  subtitle?: React.ReactNode;
  inline?: boolean;
};

export const TitleMin = ({
  title,
  subtitle,
  inline = false,
}: MinTitleProps) => {
  const subtitleArray = React.Children.toArray(subtitle);
  return (
    <Box
      style={{
        lineHeight: 1,
      }}
    >
      <Heading
        size={"2"}
        as={"h4"}
        style={{
          display: inline ? "inline" : "block",
          lineHeight: 1,
        }}
      >
        {title}
      </Heading>{" "}
      <Text
        size={"1"}
        as="span"
        style={{
          display: inline ? "inline" : "block",
          paddingLeft: inline ? "var(--space-1)" : 0,
          lineHeight: 1,
        }}
      >
        {subtitleArray.map((x, i) => (
          <React.Fragment key={i}>{x}</React.Fragment>
        ))}
      </Text>
    </Box>
  );
};
