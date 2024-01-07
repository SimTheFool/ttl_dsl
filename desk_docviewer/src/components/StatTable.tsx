import { Table } from "@radix-ui/themes";
import { ReactNode } from "react";
import styles from "./StatTable.module.css";

type Items = [ReactNode[], ...ReactNode[][]];

type StateTableProps = {
  items: Items;
  inline?: boolean;
  compact?: boolean;
};

export const StatTable = ({
  items,
  inline = false,
  compact = false,
}: StateTableProps) => {
  const headers = items?.[0];
  const [_, ...rows] = items;

  return (
    <Table.Root
      size="1"
      className={[
        styles.table,
        inline && styles.tableInline,
        compact && styles.tableCompact,
      ].join(" ")}
    >
      <Table.Header>
        <Table.Row>
          {headers.map((title, i) => (
            <Table.ColumnHeaderCell key={i}>{title}</Table.ColumnHeaderCell>
          ))}
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {rows.map((row, i) => (
          <Table.Row key={i}>
            {row.map((cell, i) => (
              <Table.Cell key={i}>{cell}</Table.Cell>
            ))}
          </Table.Row>
        ))}
      </Table.Body>
    </Table.Root>
  );
};
