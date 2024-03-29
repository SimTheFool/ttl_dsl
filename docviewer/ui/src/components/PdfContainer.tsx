"use client";
import { getA4FormatFromWidth } from "@/utils/a4format";
import { Box, Theme } from "@radix-ui/themes";
import "./PdfContainer.css";

type A4FormatProps = {
  children: React.ReactNode;
  border?: boolean;
  noBreak?: boolean;
  footer?: React.ReactNode;
};

export const PdfContainer = ({ children, footer, border = false, noBreak }: A4FormatProps) => {
  let sizes = getA4FormatFromWidth(787);

  return (
    <Theme
      className="PdfContainer"
      style={{
        width: `${sizes.width}px`,
        height: `${sizes.height}px`,
        padding: "10px",
        border: border ? "2px solid var(--gray-10)" : "unset",
        boxSizing: "border-box",
        overflow: "hidden",
        pageBreakAfter: noBreak ? "unset" : "always",
      }}
    >
      <Box
        style={{
          maxHeight: "100%",
          width: "100%",
          height: "100%",
        }}
      >
        {children}
        {footer && (
          <Box
            style={{
              position: "absolute",
              bottom: "0",
              right: "0",
            }}
          >
            {footer}
          </Box>
        )}
      </Box>
    </Theme>
  );
};