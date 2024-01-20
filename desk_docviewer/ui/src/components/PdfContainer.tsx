"use client";
import { getA4FormatFromWidth } from "@/utils/a4format";
import { Box, Theme } from "@radix-ui/themes";
import "./PdfContainer.css";

type A4FormatProps = {
  children: React.ReactNode;
  border?: boolean;
};

export const PdfContainer = ({ children, border = false }: A4FormatProps) => {
  let sizes = getA4FormatFromWidth(690);

  return (
    <Theme
      className="PdfContainer"
      style={{
        width: `${sizes.width}px`,
        height: `${sizes.height}px`,
        border: border ? "2px solid var(--gray-10)" : "unset",
        boxSizing: "border-box",
        overflow: "hidden",
      }}
    >
      <Box
        pt={"5"}
        style={{
          height: "100%",
          width: "100%",
        }}
      >
        {children}
      </Box>
      <PdfBreak />
    </Theme>
  );
};

const PdfBreak = () => {
  return (
    <div
      style={{
        pageBreakAfter: "always",
      }}
    />
  );
};
