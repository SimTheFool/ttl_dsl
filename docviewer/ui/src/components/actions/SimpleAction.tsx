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
import { Duration } from "../Icons/Duration";
import { SpellNature } from "../Icons/SpellNature";
import { SpellDistance } from "../Icons/SpellDistance";
import { SpellZone } from "../Icons/SpellZone";
import { Threshold } from "../Icons/Threshold";

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
    duration,
    threshold,
    description,
    gauge,
    ammo,
    damage,
    ranges,
    reaction,
    range,
    nature,
    zone,
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

  const infoIcons = [
    reaction && <GiLibertyWing />,
    nature && <SpellNature nature={nature} />,
    range && <SpellDistance range={range} />,
    zone && <SpellZone zone={zone} />,
  ].filter((x) => x);

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
  const isDamaging = ammo || damage;

  const firstSubtitle = isDamaging ? damageSubtitle : infosSubtitle;
  const secondSubtitle =
    isDamaging && !!infoIcons.length ? infosSubtitle : null;

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
          {description && (
            <>
              <Space />
              <ParagraphStandard>
                <TextReplaced>{description}</TextReplaced>
              </ParagraphStandard>
            </>
          )}
          {ammo_gauge && (
            <>
              <Space />
              <Gauge length={ammo_gauge} icon={<Bullet />} />
            </>
          )}
          {gauge && <Gauge length={gauge} icon={<PiDiamondLight />} />}
          {rulerGradeLabel && (
            <ParagraphStandard>
              {rulerGradScore && (
                <Ruler grade={rulerGradeLabel} inter={rulerGradScore} />
              )}
            </ParagraphStandard>
          )}
        </Box>

        <Box pl={"1"}>
          {duration && <Duration n={duration} />}
          {threshold && <Threshold n={threshold} />}
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
