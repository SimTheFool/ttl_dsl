import { Advantage } from "./Icons/Advantage";
import { Damage } from "./Icons/Damage";
import { DiceSix } from "./Icons/DiceSix";
import { Formula } from "./Icons/Formula";
import { Success } from "./Icons/Success";

const ICON_ID_REGEX = /__([A-Z0-9]+)__/;
const FORMULA_REGEX = /--([^-]+)--/;

type TextProps = { children: string };
export const TextReplaced = ({ children }: TextProps) => {
  const parts = children.split(/(__[A-Z0-9]+__)/);
  const parts2 = parts.flatMap((p) => p.split(/(--[^-]+--)/));

  const partsWithIcons = parts2.map((part, i) => {
    const iconId = part.match(ICON_ID_REGEX)?.[1];
    const formula = part.match(FORMULA_REGEX)?.[1];

    const rendered = iconId ? (
      <Icon type={iconId as any} key={i} />
    ) : formula ? (
      <Formula text={formula} />
    ) : (
      part.replace(/_/g, " ")
    );

    return rendered;
  });

  return <>{partsWithIcons}</>;
};

type IconProps = {
  type: keyof typeof iconTextsList;
};
export const Icon = ({ type }: IconProps) => {
  return iconTextsList[type];
};

const iconTextsList = {
  A1: <Advantage n={1} />,
  D1: <Advantage n={-1} />,
  SN: <Success />,
  DOM: <Damage />,
  RD: <DiceSix />,
} as const;
