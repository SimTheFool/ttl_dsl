import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";

import { Box } from "@radix-ui/themes";
import { SRCharacter } from "./character";
import { Trait } from "@/components/actions/Trait";

type EffectsProps = {
  char: SRCharacter;
};

export const Traits = ({ char }: EffectsProps) => {
  return (
    <Section>
      <MasonryGridNoSSR compact columns={4}>
        <TitleSection>Effets</TitleSection>
        {Object.entries(char.traits || {}).map(([name, trait], i) => (
          <Box key={i} pr={"2"} pb={"2"}>
            <Trait name={name} trait={trait} />
          </Box>
        ))}
      </MasonryGridNoSSR>
    </Section>
  );
};
