import { Sprite as SpriteType } from "@/app/mock/type";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";
import { StatTable } from "../StatTable";
import { CompanionBox, ErgoCompanionBox } from "./CompanionBox";

type SpriteProps = {
  name: string;
  sprite: SpriteType;
  ergo?: boolean;
};

export const Sprite = ({ name, sprite, ergo = false }: SpriteProps) => {
  const stats = sprite.stats;
  const Container = ergo ? ErgoCompanionBox : CompanionBox;

  return (
    <Container companion={sprite} name={name} type={"sprite"}>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["Firew.", "Trait.", "Corr.", "Att."],
              [
                `P+${stats.firewall}`,
                `P+${stats.traitement}`,
                `P+${stats.corruption}`,
                `P+${stats.attaque}`,
              ],
            ]}
          />
          <StatTable
            compact
            items={[
              [
                "Vie",
                <InlineMajorAction size={10} />,
                <InlineMinorAction size={12} />,
              ],
              [
                `${stats.hit?.base}+P/${stats.hit?.factor}`,
                `${stats.action_maj}`,
                `${stats.action_min}`,
              ],
            ]}
          />
        </>
      )}
    </Container>
  );
};
