import { FlexList } from "@/components/FlexList";
import { DiceSix } from "@/components/Icons/DiceSix";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { TextIndice } from "@/components/TextIndice";
import { TitleSection } from "@/components/TitleSection";
import { Character } from "@/mock/type";
import { Box, Table } from "@radix-ui/themes";

type StatsProps = {
  char: Character;
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
      <Container>
        {stats.res && (
          <StatTable
            inline
            items={[
              ["ESS", "RES", "Subm."],
              [stats.ess, stats.res, stats.submersion],
            ]}
          />
        )}
      </Container>
      <Container>
        {stats.mag && (
          <StatTable
            items={[
              ["ESS", "MAG", "Init."],
              [stats.ess, stats.mag, stats.initiation],
            ]}
          />
        )}
      </Container>
      <Container>
        {stats.resist_drain && (
          <StatTable
            items={[
              ["Res.Drain"],
              [
                <StatBlock
                  n={stats.resist_drain?.score || 0}
                  stat={stats.resist_drain?.stat}
                />,
              ],
            ]}
          />
        )}
      </Container>
      <Container>
        <StatTable
          items={[
            ["Ini"],
            [
              <>
                <StatBlock n={stats.init_dice} dice={6} />
                {"+"}
                <StatBlock n={stats.init.score} stat={stats.init?.stat} />
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
                n={stats.natural_heal.score}
                stat={stats.natural_heal.stat}
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
                n={stats.def_phy.score}
                stat={stats.def_phy.stat}
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
                n={stats.def_ment.score}
                stat={stats.def_ment.stat}
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
                n={stats.resist_phy.score}
                stat={stats.resist_phy.stat}
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
                n={stats.resist_ment.score}
                stat={stats.resist_ment.stat}
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
