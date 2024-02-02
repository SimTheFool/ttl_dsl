import { Card } from "@/components/Card";
import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { Section } from "@/components/Section";
import { Space } from "@/components/Space";
import { TextReplaced } from "@/components/Text";
import { TitleSection } from "@/components/TitleSection";
import { capitalize } from "@/utils/capitalize";
import { uncapitalize } from "@/utils/uncapitalize";
import { Box, Flex, Text } from "@radix-ui/themes";
import { ReactNode } from "react";
import { SRCharacter } from "./character";

type SkillsProps = {
  char: SRCharacter;
};

export const Skills = ({ char }: SkillsProps) => {
  let skills = Object.entries(char.skills);
  return (
    <Section>
      <MasonryGridNoSSR compact columns={2}>
        <TitleSection>Compétences</TitleSection>
        {skills.map(([name, value]) => (
          <Container key={name}>
            {value && (
              <Card>
                <SkillText
                  name={<TextReplaced>{capitalize(name)}</TextReplaced>}
                  score={value.score}
                />
                {Object.values(value.specializations || {}).map((name) => (
                  <MasterText score={2} label={name} key={name} />
                ))}
                {Object.values(value.masterizations || {}).map((name) => (
                  <MasterText score={3} label={name} key={name} />
                ))}
              </Card>
            )}
          </Container>
        ))}
        <Container>
          <Card>
            <SkillText name={capitalize("connaissances")} />
            {Object.values(char.knowledges || {}).map((name) => (
              <MasterText label={`- ${name}`} key={name} />
            ))}
          </Card>
        </Container>
      </MasonryGridNoSSR>
    </Section>
  );
};

const Container = ({ children }: { children: React.ReactNode }) => {
  return (
    <Box pr={"2"} pb={"2"} grow={"1"}>
      {children}
    </Box>
  );
};

const SkillText = ({ name, score }: { name: ReactNode; score?: number }) => {
  return (
    <Flex
      justify={"between"}
      style={{
        width: "100%",
      }}
    >
      <Text
        weight={"bold"}
        size={"2"}
        style={{
          maxWidth: "90%",
          flexShrink: 1,
          lineHeight: 1.5,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
        }}
      >
        {name}
      </Text>
      {score && (
        <Box pl={"1"} asChild>
          <Text
            weight={"bold"}
            size={"2"}
            style={{
              flexShrink: 0,
              lineHeight: 1.5,
            }}
          >
            {score}
          </Text>
        </Box>
      )}
    </Flex>
  );
};

const MasterText = ({ label, score }: { label: string; score?: number }) => {
  return (
    <Flex pl={"2"}>
      <Text
        size={"1"}
        style={{
          lineHeight: 1,
        }}
      >
        <Space inline />
      </Text>
      {score && (
        <Box pr={"1"} asChild>
          <Text
            size={"1"}
            style={{
              lineHeight: 1,
            }}
          >
            +{score}
          </Text>
        </Box>
      )}
      <Text
        size={"1"}
        style={{
          maxWidth: "90%",
          flexShrink: 1,
          lineHeight: 1,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
        }}
      >
        {uncapitalize(label)}
      </Text>
    </Flex>
  );
};
