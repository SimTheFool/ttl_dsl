import { BiSolidStar } from "react-icons/bi";
import { BaseIcon } from "./BaseIcon";

type EdgeProps = {};

export const Edge = ({}: EdgeProps) => {
  return (
    <BaseIcon size={22}>
      <BiSolidStar
        size="18"
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const EdgeLight = ({}: EdgeProps) => {
  return (
    <BaseIcon size={22}>
      <BiSolidStar
        size="14"
        style={{
          color: "var(--gray-6)",
        }}
      />
    </BaseIcon>
  );
};
