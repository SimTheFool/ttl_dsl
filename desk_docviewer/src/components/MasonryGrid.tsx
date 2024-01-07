"use client";

import { Box } from "@radix-ui/themes";
import Masonry from "masonry-layout";
import React, { useId } from "react";
import { useRef, useEffect } from "react";

type MasonryGridProps = {
  columns: number;
  children?: React.ReactNode;
  compact?: boolean;
};

export const MasonryGrid = ({
  children,
  columns,
  compact = false,
}: MasonryGridProps) => {
  const gridRef = useRef<HTMLDivElement>(null);
  const columnWidth = 100 / columns;

  const id = useId();
  const formattedId = id.replace(/[:]/g, "");
  const sizerId = `${formattedId}Sizer`;
  const itemId = `${formattedId}Item`;

  useEffect(() => {
    if (!gridRef.current) return;
    const masonry = new Masonry(gridRef.current, {
      itemSelector: `.${itemId}`,
      columnWidth: `.${sizerId}`,
      percentPosition: true,
    });

    return () => {
      masonry.destroy?.();
    };
  }, [gridRef.current, sizerId, itemId]);

  return (
    <Box ref={gridRef}>
      <Box
        className={sizerId}
        style={{
          visibility: "hidden",
          width: `${columnWidth}%`,
        }}
      />

      {React.Children.map(children, (child, i) => {
        return (
          <Box
            key={i}
            className={itemId}
            style={{
              width: `${columnWidth}%`,
            }}
            px={compact ? "0" : "1"}
            pb={compact ? "0" : "1"}
          >
            {child}
          </Box>
        );
      })}
    </Box>
  );
};
