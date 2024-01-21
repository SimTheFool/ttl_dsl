import { StatTable } from "../StatTable";
import { CompanionBox, ErgoCompanionBox } from "./CompanionBox";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";
import { Monitor } from "../Monitor";
import { SRCompanion } from "@/app/SRDocument/character";
import { Box } from "@radix-ui/themes";

type OtherCompanionProps = {
  name: string;
  companion: SRCompanion;
  ergo?: boolean;
  slot?: boolean;
};

export const OtherCompanion = ({
  name,
  companion: otherCompanion,
  ergo = false,
  slot = true,
}: OtherCompanionProps) => {
  const primary = otherCompanion.stats_primary;
  const Container = ergo ? ErgoCompanionBox : CompanionBox;

  return (
    <Container companion={otherCompanion} name={name} noSlot={!slot}>
      {primary && (
        <>
          <StatTable
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
              [!!primary.hit_formula ? `${primary.hit_formula}` : null],
            ]}
          />
          {primary.hit && (
            <Monitor columns={primary.hit} hit={primary.hit} alwaysCurable />
          )}
        </>
      )}
    </Container>
  );
};
