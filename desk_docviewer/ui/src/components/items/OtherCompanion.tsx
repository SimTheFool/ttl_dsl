import { OtherCompanion as OtherCompanionType } from "@/mock/type";
import { StatTable } from "../StatTable";
import { CompanionBox, ErgoCompanionBox } from "./CompanionBox";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";
import { Monitor } from "../Monitor";

type OtherCompanionProps = {
  name: string;
  otherCompanion: OtherCompanionType;
  ergo?: boolean;
};

export const OtherCompanion = ({
  name,
  otherCompanion,
  ergo = false,
}: OtherCompanionProps) => {
  const stats = otherCompanion.stats;
  const Container = ergo ? ErgoCompanionBox : CompanionBox;

  return (
    <Container companion={otherCompanion} name={name} type={"esprit"} noSlot>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["con", "agi", "rea", "for"],
              [stats.con, stats.agi, stats.rea, stats.for],
            ]}
          />
          <StatTable
            compact
            items={[
              ["vol", "log", "int", "cha"],
              [stats.vol, stats.log, stats.int, stats.cha],
            ]}
          />
          <StatTable
            compact
            items={[
              [
                "Puiss.",
                <InlineMajorAction size={10} />,
                <InlineMinorAction size={12} />,
              ],
              [`${stats.pui}`, `${stats.action_maj}`, `${stats.action_min}`],
            ]}
          />
          <Monitor columns={stats.hit} hit={stats.hit} alwaysCurable />
        </>
      )}
    </Container>
  );
};
