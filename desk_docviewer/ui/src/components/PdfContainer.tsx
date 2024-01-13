"use client";

import { Box, Theme } from "@radix-ui/themes";
import "./PdfContainer.css";
import { getA4FormatFromWidth } from "@/utils/a4format";
import { useEffect, useState } from "react";

type A4FormatProps = {
  children: React.ReactNode;
  border?: boolean;
  footer?: React.ReactNode;
};

export const PdfContainer = ({
  children,
  footer,
  border = false,
}: A4FormatProps) => {
  const [baseWidth, setBaseWidth] = useState<number>(750);
  let sizes = getA4FormatFromWidth(baseWidth - 5);

  useEffect(() => {
    if (!window) return;
    setBaseWidth(window.innerWidth);
  }, []);

  return (
    <Theme
      style={{
        width: `${sizes.width}px`,
        height: `${sizes.height}px`,
        border: border ? "2px solid var(--gray-10)" : "unset",
        boxSizing: "border-box",
        overflow: "hidden",
      }}
    >
      <Box
        pt={"8"}
        px={"2"}
        style={{
          height: "100%",
          width: "100%",
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

export const PdfBreak = () => {
  return (
    <div
      style={{
        pageBreakAfter: "always",
      }}
    />
  );
};
