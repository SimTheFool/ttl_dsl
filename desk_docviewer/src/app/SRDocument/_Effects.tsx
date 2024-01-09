import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Effect } from "@/components/actions/Effect";
import { Character } from "@/app/mock/type";
import { Box } from "@radix-ui/themes";

type EffectsProps = {
  char: Character;
};

export const Effects = ({ char }: EffectsProps) => {
  return (
    <Section>
      <MasonryGridNoSSR compact columns={4}>
        <TitleSection>Effets</TitleSection>
        {char.effects?.map((e, i) => (
          <Box key={i} pr={"2"} pb={"2"}>
            <Effect effect={e} />
          </Box>
        ))}
      </MasonryGridNoSSR>
    </Section>
  );
};
