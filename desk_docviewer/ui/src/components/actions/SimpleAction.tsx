import { SRAction } from "@/app/SRDocument/character";
import { capitalize } from "@/utils/capitalize";
import { interleave } from "@/utils/interleave";
import { Box, Flex } from "@radix-ui/themes";
import { GiLibertyWing } from "react-icons/gi";
import { PiDiamondLight } from "react-icons/pi";
import { Card } from "../Card";
import { Gauge } from "../Gauge";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { Bullet } from "../Icons/Bullet";
import { Damage } from "../Icons/Damage";
import { Maintained } from "../Icons/Maintained";
import { ParagraphStandard } from "../ParagraphStandard";
import { Ruler } from "../Ruler";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";

type BaseActionProps = {
  name: string;
  action: SRAction;
  baseRanges?: {
    contact: { label?: number };
    near: { label?: number };
    short: { label?: number };
    mid: { label?: number };
    far: { label?: number };
  };
};
export const SimpleAction = ({
  name,
  action: {
    test,
    major,
    minor,
    description,
    gauge,
    ammo,
    damage,
    ranges,
    reaction,
    maintained,
    ammo_gauge,
  },
  baseRanges,
}: BaseActionProps) => {
  const validRanges = (
    ["contact", "near", "short", "mid", "far"] as const
  ).filter((r) => baseRanges?.[r]?.label);

  const rulerGradeLabel =
    baseRanges &&
    validRanges.map((r) => (baseRanges[r] as any).label as string);
  const rulerGradScore = ranges && validRanges.map((r) => ranges[r]);

  const infoIcons = [reaction && <GiLibertyWing />].filter((x) => x);

  const damageSubtitle = (
    <>
      {!!ammo && (
        <span
          style={{
            fontWeight: "bold",
          }}
        >
          {ammo}
          <Bullet />
        </span>
      )}
      {!!damage && (
        <>
          <span
            style={{
              fontWeight: "bold",
            }}
          >
            {damage}
          </span>
          <Damage />
        </>
      )}
    </>
  );

  const infosSubtitle = interleave(infoIcons, <Space inline />);

  const firstSubtitle = !!ammo ? damageSubtitle : infosSubtitle;
  const secondSubtitle = !!ammo && !!infoIcons.length ? infosSubtitle : null;

  return (
    <Card title={test}>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={<TextReplaced>{capitalize(name || "")}</TextReplaced>}
            inline
            subtitle={firstSubtitle}
          />
          {secondSubtitle && <TitleMin inline subtitle={secondSubtitle} />}
          {ammo_gauge && <Gauge length={ammo_gauge} icon={<Bullet />} />}
          {gauge && <Gauge length={gauge} icon={<PiDiamondLight />} />}
          {rulerGradeLabel && (
            <ParagraphStandard>
              {rulerGradScore && (
                <Ruler grade={rulerGradeLabel} inter={rulerGradScore} />
              )}
            </ParagraphStandard>
          )}
          {description && (
            <>
              <Space />
              <ParagraphStandard>
                <TextReplaced>{description}</TextReplaced>
              </ParagraphStandard>
            </>
          )}
        </Box>

        <Box pl={"1"}>
          {maintained && <Maintained />}
          {!!major &&
            Array.from({ length: major }).map((_, i) => (
              <MajorAction key={i} />
            ))}
          {!!minor &&
            Array.from({ length: minor }).map((_, i) => (
              <MinorAction key={i} />
            ))}
        </Box>
      </Flex>
    </Card>
  );
};
