import { StatTable } from "../StatTable";
import { CompanionBox, ErgoCompanionBox } from "./CompanionBox";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";
import { Monitor } from "../Monitor";
import { SRCompanion } from "@/app/SRDocument/character";
import { Box } from "@radix-ui/themes";
import { Slot } from "./Slot";
import { Space } from "../Space";

type OtherCompanionProps = {
  name: string;
  companion: SRCompanion;
  ergo?: boolean;
  slot?: boolean;
};

export const Companion = ({
  name,
  companion: otherCompanion,
  ergo = false,
}: OtherCompanionProps) => {
  const primary = otherCompanion.stats_primary;
  const secondary = otherCompanion.stats_secondary || {};
  const dynamic = otherCompanion.dynamic;
  const Container = ergo ? ErgoCompanionBox : CompanionBox;

  const aside = dynamic ? (
    <Slot size="L" note={"pui. serv. vie"} />
  ) : (
    primary && <Monitor columns={primary.hit} hit={primary.hit} alwaysCurable />
  );

  const stats = Object.entries(secondary).map(([name, value]) => (
    <Box
      pr={"1"}
      style={{
        display: "inline",
      }}
    >
      <StatTable
        items={[
          [name],
          [
            dynamic
              ? `X${value < 0 ? value : value == 0 ? "" : `+${value}`}`
              : value,
          ],
        ]}
        inline
      />
    </Box>
  ));

  return (
    <Container companion={otherCompanion} name={name} aside={aside}>
      {primary && (
        <>
          <Space />
          <StatTable
            inline
            compact
            items={[
              [
                <>
                  {Array.from({ length: primary.major }).map((_, i) => (
                    <Box display={"inline-block"} key={i}>
                      <InlineMajorAction size={14} />
                    </Box>
                  ))}
                  {Array.from({ length: primary.minor }).map((_, i) => (
                    <Box display={"inline-block"} key={i}>
                      <InlineMinorAction size={12} />
                    </Box>
                  ))}
                </>,
              ],
              [
                dynamic ? (
                  `${primary.hit}+X/${dynamic}`
                ) : (
                  <Box
                    style={{
                      visibility: "hidden",
                    }}
                  >
                    {primary.hit}
                  </Box>
                ),
              ],
            ]}
          />
          <Space inline />
          {stats}
        </>
      )}
    </Container>
  );
};
