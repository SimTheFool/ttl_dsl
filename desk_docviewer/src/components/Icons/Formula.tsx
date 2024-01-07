import { Box } from "@radix-ui/themes";

type FormulaProps = {
  text: string;
};
export const Formula = ({ text }: FormulaProps) => {
  return (
    <Box
      style={{
        display: "inline",
        fontStyle: "normal",
        fontWeight: "bold",
        margin: "0 var(--space-1)",
        fontSize: "calc(13px * var(--scaling))",
      }}
    >
      {text}
    </Box>
  );
};
