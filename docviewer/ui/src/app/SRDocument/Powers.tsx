import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { PdfContainer } from "@/components/PdfContainer";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { SimpleAction } from "@/components/actions/SimpleAction";
import { Box, Flex } from "@radix-ui/themes";
import { ReactNode, useMemo } from "react";
import { SRAction, SRCharacter } from "./character";
import { Companion } from "@/components/items/Companion";

type Props = {
  char: SRCharacter;
};

export default function Powers({ char }: Props) {
  const weightPowers = Object.keys(char.powers || {}).length;

  const weightCompanions = Object.entries(char.companions || {}).reduce(
    (acc, [_, companion]) => {
      const wSkills = Object.keys(companion.skills || {}).length;
      const wTrais = Object.keys(companion.traits || {}).length;
      return acc + wSkills + wTrais + 1;
    },
    0
  );

  const weight = weightPowers + weightCompanions;

  if (weight < 20)
    return (
      <>
        <PdfContainer footer={"POUVOIRS ET COMPAGNONS"}>
          <PowersAndCompanions char={char} />
        </PdfContainer>
        <PdfContainer footer={"ACTIONS COMMUNES"}>
          <CommonActionOnly char={char} />
        </PdfContainer>
        <PdfContainer footer={"ACTIONS MAGIQUES"}>
          <MagicActionOnly char={char} />
        </PdfContainer>
      </>
    );

  return (
    <>
      <PdfContainer footer={"COMPAGNONS"}>
        <CompanionOnly char={char} />
      </PdfContainer>
      <PdfContainer footer={"POUVOIRS"}>
        <PowersOnly char={char} />
      </PdfContainer>
      <PdfContainer footer={"ACTIONS COMMUNES"}>
        <CommonActionOnly char={char} />
      </PdfContainer>
      <PdfContainer footer={"ACTIONS MAGIQUES"}>
        <MagicActionOnly char={char} />
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

  return <ActionsList actions={actions} title="Actions Communes" columns={3} />;
};

const MagicActionOnly = ({ char }: { char: SRCharacter }) => {
  return (
    <ActionsList
      actions={Object.entries(char.actions_magic || {})}
      title="Actions Magiques"
      columns={2}
    />
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

const ActionsList = ({
  actions,
  title,
  columns = 3,
}: {
  actions: [string, SRAction][];
  title: string;
  columns?: number;
}) => {
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
          maxWidth: `${100 / columns}%`,
        }}
      >
        <TitleSection>{title}</TitleSection>
        <Space />
      </Box>

      {actions.map(([name, action]) => {
        return (
          <Box
            style={{
              maxWidth: `${100 / columns}%`,
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
