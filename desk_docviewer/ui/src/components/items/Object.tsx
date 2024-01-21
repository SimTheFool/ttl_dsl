import { SRObject } from "@/app/SRDocument/character";
import { interleave } from "@/utils/interleave";
import { Box, Flex } from "@radix-ui/themes";
import { Hand } from "../Icons/Hand";
import { MasonryGridNoSSR } from "../MasonryGridNoSSR";
import { ParagraphStandard } from "../ParagraphStandard";
import { Price } from "../Price";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import styles from "./ItemCard.module.css";
import { Slot } from "./Slot";
import { SimpleAction } from "../actions/SimpleAction";
import { Ruler } from "../Ruler";
import { Monitor } from "../Monitor";
import { Card } from "../Card";
import { Quality } from "../Quality";

const statusTrad = {
  free: null,
  licenced: "licencié",
  illegal: "illégal",
};

type ObjectProps = {
  object: SRObject;
  name: string;
  noHand?: boolean;
  unit?: boolean;
};

export const Object = ({
  object: {
    slots,
    quantity,
    manufacturer,
    price,
    price_unit,
    status,
    concealment,
    description,
    quality,
    actions,
    ranges,
    stats_primary,
  },
  name,
  noHand = false,
  unit = false,
}: ObjectProps) => {
  const slotsParts = window.Object.values(slots || {})
    .sort((a, b) => {
      if (!a.name) return -1;
      if (!b.name) return 1;
      return a.name.localeCompare(b.name);
    })
    .map((slot) => {
      return (
        <Slot
          key={slot.name}
          size={slot.size}
          concealment={slot.concealment}
          note={slot.name}
        ></Slot>
      );
    });

  const actionsParts = window.Object.entries(actions || {})
    .sort(([, a], [, b]) => {
      if (!a.ammo) return -1;
      if (!b.ammo) return 1;
      return a.ammo - b.ammo;
    })
    .map(([name, action]) => {
      return (
        <SimpleAction
          key={name}
          name={name}
          action={action}
          baseRanges={ranges}
        ></SimpleAction>
      );
    });

  const bottomChildrenWithSlots = [...actionsParts, ...slotsParts];
  const bottomItemNb = bottomChildrenWithSlots.length;

  const titleParts = [
    name.toUpperCase(),
    quantity && quantity > 1 ? `x${quantity}` : null,
  ]
    .filter((x) => x)
    .map((x: any) => <TextReplaced>{x}</TextReplaced>);
  const title = interleave(titleParts, <Space inline />);

  const subtitleParts = [
    <Price price={unit ? price_unit : price} unit={unit} />,
    quality && <Quality quality={quality} />,
    <>{statusTrad[status]}</>,
  ].filter((x) => x);
  const subtitle = interleave(subtitleParts, <Space inline />);

  const validRanges = (
    ["contact", "near", "short", "mid", "far"] as const
  ).filter((r) => ranges?.[r]?.label);
  const rulerGradeLabel =
    ranges && validRanges.map((r) => (ranges[r] as any).label as string);

  return (
    <Box>
      <Flex className={bottomItemNb > 0 ? styles.noBorderBottom : ""}>
        <Card>
          <Flex justify={"between"}>
            <Box>
              <TitleMin inline title={title} subtitle={manufacturer} />
              <TitleMin subtitle={subtitle} />
            </Box>

            {!noHand && (
              <Box pl={"1"}>
                <Hand n={concealment} />
              </Box>
            )}
          </Flex>

          <ParagraphStandard>
            {description && (
              <>
                <Space />
                <TextReplaced>{description}</TextReplaced>
                <Space />
              </>
            )}
            {rulerGradeLabel && <Ruler grade={rulerGradeLabel} />}
            {stats_primary?.hit && (
              <Monitor
                columns={stats_primary.hit}
                hit={stats_primary.hit}
                alwaysCurable
              />
            )}
          </ParagraphStandard>
        </Card>
      </Flex>
      <MasonryGridNoSSR compact columns={1}>
        {bottomChildrenWithSlots.map((child, i) => (
          <Box key={i} className={i == 0 ? "" : styles.bottom}>
            {child}
          </Box>
        ))}
      </MasonryGridNoSSR>
    </Box>
  );
};
