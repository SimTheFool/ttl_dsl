import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { PdfContainer } from "@/components/PdfContainer";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { SimpleAction } from "@/components/actions/SimpleAction";
import { Box, Flex } from "@radix-ui/themes";
import { ReactNode, useMemo } from "react";
import { SRCharacter } from "./character";
import { Companion } from "@/components/items/Companion";

type Props = {
  char: SRCharacter;
};

export default function Powers({ char }: Props) {
  const weight =
    Object.keys(char.powers || {}).length +
    Object.keys(char.companions || {}).length;

  if (weight < 20)
    return (
      <>
        <PdfContainer>
          <PowersAndCompanions char={char} />
        </PdfContainer>
        <PdfContainer>
          <CommonActionOnly char={char} />
        </PdfContainer>
      </>
    );

  return (
    <>
      <PdfContainer>
        <PowersOnly char={char} />
      </PdfContainer>
      <PdfContainer>
        <CompanionOnly char={char} />
      </PdfContainer>
      <PdfContainer>
        <CommonActionOnly char={char} />
      </PdfContainer>
    </>
  );
}

const PowersOnly = ({ char }: { char: SRCharacter }) => {
  return (
    <MasonryGridNoSSR columns={3}>
      <Box>
        <TitleSection>Pouvoirs</TitleSection>
        <Space />
      </Box>

      {Object.entries(char.powers || {}).map(([name, power]) => {
        return (
          <Container key={name}>
            <SimpleAction name={name} action={power} />
          </Container>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const CommonActionOnly = ({ char }: { char: SRCharacter }) => {
  const actions = useMemo(() => {
    return Object.entries(char.actions_common || {}).sort(([, a], [, b]) => {
      if (a.reaction && !b.reaction) return 1;
      if (!a.reaction && b.reaction) return -1;
      return 0;
    });
  }, [char.actions_common]);

  return (
    <Flex
      style={{
        flexWrap: "wrap",
        flexDirection: "column",
        maxHeight: "100%",
      }}
    >
      <Box
        style={{
          maxWidth: "33%",
        }}
      >
        <TitleSection>Actions Communes</TitleSection>
        <Space />
      </Box>

      {actions.map(([name, action]) => {
        return (
          <Box
            style={{
              maxWidth: "33%",
            }}
            key={name}
          >
            <Container key={name}>
              <SimpleAction name={name} action={action} />
            </Container>
          </Box>
        );
      })}
    </Flex>
  );
};

const CompanionOnly = ({ char }: { char: SRCharacter }) => {
  return (
    <MasonryGridNoSSR columns={1}>
      <Box>
        <TitleSection>Compagnons</TitleSection>
        <Space />
      </Box>

      {Object.entries(char.companions || {}).map(([name, companion]) => {
        return (
          <Container key={name}>
            <Companion name={name} companion={companion} ergo />
          </Container>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const PowersAndCompanions = ({ char }: { char: SRCharacter }) => {
  return (
    <>
      <Box>
        <TitleSection>Pouvoirs et compagnons</TitleSection>
        <Space />
      </Box>

      <MasonryGridNoSSR columns={3}>
        {Object.entries(char.powers || {}).map(([name, power]) => {
          return (
            <Container key={name}>
              <SimpleAction name={name} action={power} />
            </Container>
          );
        })}
        {Object.entries(char.companions || {}).map(([name, companion]) => {
          return (
            <Container key={name}>
              <Companion name={name} companion={companion} />
            </Container>
          );
        })}
      </MasonryGridNoSSR>
    </>
  );
};

const Container = ({ children }: { children: ReactNode }) => {
  return (
    <Box pb={"2"} pr={"2"}>
      {children}
    </Box>
  );
};
