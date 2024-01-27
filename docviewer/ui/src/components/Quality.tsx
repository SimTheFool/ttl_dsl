import { Space } from "./Space";

export const Quality = ({
  quality,
  force = false,
}: {
  quality: number;
  force?: boolean;
}) => {
  if (!force && quality <= 1) return null;
  return <>i{quality}</>;
};
