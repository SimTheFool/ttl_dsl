import { Box } from "@radix-ui/themes";

type SpaceProps = {
  inline?: boolean;
};

export const Space = ({ inline }: SpaceProps) => {
  return (
    <Box
      pt={inline ? "0" : "1"}
      pr={inline ? "1" : "0"}
      style={{
        display: inline ? "inline-block" : "block",
      }}
    />
  );
};
