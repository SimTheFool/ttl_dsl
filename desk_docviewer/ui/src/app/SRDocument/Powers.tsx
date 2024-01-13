import { LineBlack } from "@/components/Line";
import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { PdfContainer } from "@/components/PdfContainer";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { RitualAction } from "@/components/actions/RitualAction";
import { SimpleAction } from "@/components/actions/SimpleAction";
import { SpellAction } from "@/components/actions/SpellAction";
import { OtherCompanion } from "@/components/items/OtherCompanion";
import { Spirit } from "@/components/items/Spirit";
import { Sprite } from "@/components/items/Sprite";
import { Character } from "@/mock/type";
import { getCharWeights } from "@/utils/getWeights";
import { Box } from "@radix-ui/themes";
import { ReactNode } from "react";

type Props = {
  char: Character;
};

export default function Powers({ char }: Props) {
  const charWeight = getCharWeights(char);
  const pageWeight = charWeight.powers + charWeight.companions;

  if (pageWeight < 20)
    return (
      <PdfContainer footer={"POUVOIRS"}>
        <AllInOne char={char} />
      </PdfContainer>
    );

  return (
    <>
      <PdfContainer footer={"POUVOIRS"}>
        <ActionOnly char={char} />
      </PdfContainer>
      <PdfContainer footer={"COMPAGNONS"}>
        <CompanionOnly char={char} />
      </PdfContainer>
    </>
  );
}

const ActionOnly = ({ char }: { char: Character }) => {
  return (
    <MasonryGridNoSSR columns={3}>
      <Box>
        <TitleSection>Pouvoirs</TitleSection>
        <Space />
      </Box>

      {Object.entries(char.complex_forms || {}).map(([name, form]) => {
        return (
          <Container key={name}>
            <SimpleAction name={name} action={form} type={form.type} />
          </Container>
        );
      })}
      {Object.entries(char.spells || {}).map(([name, spell]) => {
        return (
          <Container key={name}>
            <SpellAction name={name} action={spell} />
          </Container>
        );
      })}
      {Object.entries(char.rituals || {}).map(([name, ritual]) => {
        return (
          <Container key={name}>
            <RitualAction name={name} action={ritual} />
          </Container>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const CompanionOnly = ({ char }: { char: Character }) => {
  return (
    <MasonryGridNoSSR columns={1}>
      <Box>
        <TitleSection>Compagnons</TitleSection>
        <Space />
      </Box>

      {Object.entries(char.sprites || {}).map(([name, sprite]) => {
        return (
          <Container key={name}>
            <Sprite name={name} sprite={sprite} key={name} ergo />
          </Container>
        );
      })}

      <LineBlack />

      {Object.entries(char.spirits || {}).map(([name, spirit]) => {
        return (
          <Container key={name}>
            <Spirit name={name} spirit={spirit} key={name} ergo />
          </Container>
        );
      })}

      <LineBlack />

      {Object.entries(char.other_companions || {}).map(([name, companion]) => {
        return (
          <Container key={name}>
            <OtherCompanion
              name={name}
              otherCompanion={companion}
              key={name}
              ergo
            />
          </Container>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const AllInOne = ({ char }: { char: Character }) => {
  return (
    <MasonryGridNoSSR columns={3}>
      <Box>
        <TitleSection>Pouvoirs et compagnons</TitleSection>
        <Space />
      </Box>
      {Object.entries(char.sprites || {}).map(([name, sprite]) => {
        return (
          <Container key={name}>
            <Sprite name={name} sprite={sprite} key={name} />
          </Container>
        );
      })}
      {Object.entries(char.spirits || {}).map(([name, spirit]) => {
        return (
          <Container key={name}>
            <Spirit name={name} spirit={spirit} key={name} />
          </Container>
        );
      })}
      {Object.entries(char.other_companions || {}).map(([name, companion]) => {
        return (
          <Container key={name}>
            <OtherCompanion name={name} otherCompanion={companion} key={name} />
          </Container>
        );
      })}

      {Object.entries(char.complex_forms || {}).map(([name, form]) => {
        return (
          <Container key={name}>
            <SimpleAction name={name} action={form} type={form.type} />
          </Container>
        );
      })}
      {Object.entries(char.spells || {}).map(([name, spell]) => {
        return (
          <Container key={name}>
            <SpellAction name={name} action={spell} />
          </Container>
        );
      })}
      {Object.entries(char.rituals || {}).map(([name, ritual]) => {
        return (
          <Container key={name}>
            <RitualAction name={name} action={ritual} />
          </Container>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const Container = ({ children }: { children: ReactNode }) => {
  return (
    <Box pb={"2"} pr={"2"}>
      {children}
    </Box>
  );
};
