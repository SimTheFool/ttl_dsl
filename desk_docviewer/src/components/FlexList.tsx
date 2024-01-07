import { Box, Flex } from "@radix-ui/themes";
import React from "react";

type FlexListProps = {
  children?: React.ReactNode;
};

export const FlexList = ({ children }: FlexListProps) => {
  return (
    <Flex wrap={"wrap"} align={"stretch"}>
      {children}
    </Flex>
  );
};
