import { Box, Grid, Heading } from "@radix-ui/themes";
import { CSSProperties, ReactNode } from "react";
import { FaSkull } from "react-icons/fa";
import styles from "./Monitor.module.css";
import { BaseIcon } from "./Icons/BaseIcon";

type MonitorProps = {
  hit: number;
  columns: number;
  title?: ReactNode;
  alwaysCurable?: boolean;
};

export const Monitor = ({
  hit,
  title,
  columns,
  alwaysCurable = false,
}: MonitorProps) => {
  return (
    <Box>
      <Heading
        size={"1"}
        as={"h3"}
        style={{
          display: "block",
          whiteSpace: "nowrap",
        }}
      >
        {title}
      </Heading>
      <Grid columns={`${columns}`} gap="0" className={styles.monitor}>
        {Array.from({ length: hit }).map((_, i) => (
          <HitBox key={i} hideIcon={alwaysCurable} />
        ))}
      </Grid>
    </Box>
  );
};

const HitBox = ({ hideIcon = false }: { hideIcon?: boolean }) => {
  return (
    <Box className={styles.box}>
      <Box className={styles.incurable}>
        {!hideIcon && (
          <BaseIcon>
            <FaSkull
              style={{
                color: "var(--gray-11)",
              }}
            />
          </BaseIcon>
        )}
      </Box>
    </Box>
  );
};
