import { DiceSix } from "@/components/Icons/DiceSix";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { TextIndice } from "@/components/TextIndice";
import { TitleSection } from "@/components/TitleSection";
import { Box } from "@radix-ui/themes";
import { SRCharacter } from "./character";

type StatsProps = {
  char: SRCharacter;
};

export const Stats = ({ char }: StatsProps) => {
  const stats = char.stats;
  return (
    <Section title={<TitleSection>Stats</TitleSection>}>
      <Box
        pr={"2"}
        display={"inline"}
        style={{
          float: "left",
        }}
      >
        <StatTable
          items={[
            ["CON", "AGI", "REA", "FOR"],
            [
              <StatBlock n={stats.con} mod={stats.con_mod} />,
              <StatBlock n={stats.agi} mod={stats.agi_mod} />,
              <StatBlock n={stats.rea} mod={stats.rea_mod} />,
              <StatBlock n={stats.for} mod={stats.for_mod} />,
            ],
          ]}
        />
        <StatTable
          items={[
            ["VOL", "LOG", "INT", "CHA"],
            [
              <StatBlock n={stats.vol} mod={stats.vol_mod} />,
              <StatBlock n={stats.log} mod={stats.log_mod} />,
              <StatBlock n={stats.int} mod={stats.int_mod} />,
              <StatBlock n={stats.cha} mod={stats.cha_mod} />,
            ],
          ]}
        />
        {stats.res && (
          <StatTable
            items={[
              ["Firew.", "Trait.", "Corr.", "Att."],
              [
                <StatBlock n={stats.firewall || 0} />,
                <StatBlock n={stats.traitement || 0} />,
                <StatBlock n={stats.corruption || 0} />,
                <StatBlock n={stats.attaque || 0} />,
              ],
            ]}
          />
        )}
      </Box>
      {stats.res && (
        <Container>
          <StatTable
            inline
            items={[
              ["ESS", "RES", "Subm."],
              [stats.ess, stats.res, stats.submersion],
            ]}
          />
        </Container>
      )}
      {stats.mag && (
        <Container>
          <StatTable
            items={[
              ["ESS", "MAG", "Init."],
              [stats.ess, stats.mag, stats.initiation],
            ]}
          />
        </Container>
      )}
      {stats.resist_drain && (
        <Container>
          <StatTable
            items={[
              ["Res.Drain"],
              [
                <StatBlock
                  n={stats.resist_drain?.value || 0}
                  stat={stats.resist_drain?.metas}
                />,
              ],
            ]}
          />
        </Container>
      )}
      <Container>
        <StatTable
          items={[
            ["Ini"],
            [
              <>
                <StatBlock n={stats.init_dice} dice={6} />
                {"+"}
                <StatBlock
                  n={stats.init_base.value}
                  stat={stats.init_base.metas}
                />
              </>,
            ],
          ]}
        />
      </Container>
      <Container>
        <StatTable
          items={[
            ["Gué.Nat"],
            [
              <StatBlock
                n={stats.heal.value}
                stat={stats.heal.metas}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
      <Container>
        <StatTable
          items={[
            ["Def.Phy"],
            [
              <StatBlock
                n={stats.def_phy.value}
                stat={stats.def_phy.metas}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
      <Container>
        <StatTable
          items={[
            ["Def.Ment"],
            [
              <StatBlock
                n={stats.def_ment.value}
                stat={stats.def_ment.metas}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
      <Container>
        <StatTable
          items={[
            ["Res.Phy"],
            [
              <StatBlock
                n={stats.resist_phy.value}
                stat={stats.resist_phy.metas}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
      <Container>
        <StatTable
          items={[
            ["Res.Ment"],
            [
              <StatBlock
                n={stats.resist_ment.value}
                stat={stats.resist_ment.metas}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
    </Section>
  );
};

type StatBlockProps = {
  n: number;
  mod?: number;
  stat?: string[];
  dice?: number;
};

const StatBlock = ({ n, mod, stat, dice }: StatBlockProps) => {
  return (
    <>
      {dice ? (
        <>
          <span>{n}</span>
          <DiceSix />
        </>
      ) : (
        n
      )}
      {!!mod ? <>({mod})</> : null}{" "}
      <TextIndice>
        {stat && <>{stat.map((s) => s.toUpperCase()).join("-")} </>}
      </TextIndice>
    </>
  );
};

const Container = ({ children }: { children: React.ReactNode }) => {
  return (
    <Box
      pr={"2"}
      style={{
        display: "inline-block",
      }}
    >
      {children}
    </Box>
  );
};
