import { Box } from "@radix-ui/themes";

export const Line = () => {
  return (
    <Box
      style={{
        width: "100%",
        height: "calc(25px * var(--scaling)",
        borderBottom: "1px solid var(--gray-8)",
      }}
    />
  );
};

export const LineBlack = () => {
  return (
    <Box
      style={{
        width: "100%",
        height: "1px",
        borderBottom: "1px solid black",
      }}
    />
  );
};
