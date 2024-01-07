import { pdfsConfig } from "@/utils/config";
import { Box, Theme } from "@radix-ui/themes";
import "./PdfContainer.css";

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
  return (
    <Theme
      style={{
        width: `${pdfsConfig.size.width}px`,
        height: `${pdfsConfig.size.height}px`,
        border: border ? "2px solid var(--gray-10)" : "unset",
        boxSizing: border ? "content-box" : "border-box",
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
        breakAfter: "always",
      }}
    />
  );
};
