import { Tech as TechType } from "@/app/mock/type";
import { StatTable } from "../StatTable";
import { ItemCard } from "./ItemCard";

type WeaponProps = { tech: TechType; name: string };

export const Tech = ({ tech, name }: WeaponProps) => {
  return (
    <ItemCard item={tech} name={name}>
      {{
        inner: tech.stats && (
          <StatTable
            compact
            items={[
              ["Firew.", "Trait.", "Corr.", "Att."],
              [
                tech.stats.firewall,
                tech.stats.traitement,
                tech.stats.corruption,
                tech.stats.attaque,
              ],
            ]}
          />
        ),
      }}
    </ItemCard>
  );
};
