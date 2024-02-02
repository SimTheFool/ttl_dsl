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

  const nonNullRows = rows.filter((row) => row.some((cell) => cell !== null));

  return (
    <table
      className={[
        styles.table,
        inline && styles.tableInline,
        compact && styles.tableCompact,
      ].join(" ")}
    >
      <thead>
        <tr>
          {headers.map((title, i) => (
            <th
              key={i}
              style={{
                borderBottom: !!nonNullRows.length ? "1px solid black" : "none",
                fontSize: "13px",
              }}
            >
              {title}
            </th>
          ))}
        </tr>
      </thead>
      {!!nonNullRows.length && (
        <tbody>
          {nonNullRows.map((row, i) => (
            <tr key={i}>
              {row.map((cell, i) => (
                <td key={i}>{cell}</td>
              ))}
            </tr>
          ))}
        </tbody>
      )}
    </table>
  );
};
