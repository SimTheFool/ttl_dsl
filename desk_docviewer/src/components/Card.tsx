import { Box, Heading, Card as RadCard } from "@radix-ui/themes";
import React from "react";
import styles from "./Card.module.css";

type CardProps = {
  children?: React.ReactNode;
  title?: React.ReactNode;
  note?: React.ReactNode;
  style?: React.CSSProperties;
};

export const Card = ({ title, children, note, style }: CardProps) => {
  return (
    <RadCard className={styles.card} style={style}>
      {title && (
        <Box className={styles.cardTitle} px={"1"}>
          <Heading size={"1"} as={"h3"}>
            {title}
          </Heading>
        </Box>
      )}

      {note && (
        <Box className={styles.cardNote} px={"1"} asChild>
          <Heading size={"1"} as={"h3"} weight={"light"}>
            {note}
          </Heading>
        </Box>
      )}
      {children}
    </RadCard>
  );
};
