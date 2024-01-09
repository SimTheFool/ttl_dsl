import { Box } from "@radix-ui/themes";
import styles from "./BaseIcon.module.css";

type BaseIconProps = {
  children?: React.ReactNode;
  size?: number;
  inline?: boolean;
};

export const BaseIcon = ({ children, size, inline = false }: BaseIconProps) => {
  const sizeStyle = { "--icon-size": `${size || 14}px` } as React.CSSProperties;
  return (
    <Box
      className={styles.baseIcon}
      style={{
        ...sizeStyle,
        display: inline ? "inline-block" : "block",
      }}
    >
      {children}
    </Box>
  );
};
