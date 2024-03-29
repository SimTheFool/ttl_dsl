import { Card } from "@/components/Card";
import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { Section } from "@/components/Section";
import { Space } from "@/components/Space";
import { TitleMin } from "@/components/TitleMin";
import { TitleSection } from "@/components/TitleSection";
import { Box, Flex, Heading } from "@radix-ui/themes";
import { Fragment } from "react";
import { SRCharacter, SRIdentity as SRIdentity } from "./character";

type IdentitiesProps = {
  char: SRCharacter;
};

export const Identities = ({ char }: IdentitiesProps) => {
  let identities = Object.values(char.identities || {}).sort((a, b) => {
    if (a.name && !b.name) {
      return 1;
    }
    if (!a.name && b.name) {
      return -1;
    }
    return 0;
  });
  return (
    <Section title={<TitleSection>Identités</TitleSection>}>
      {identities.map((i, index) => (
        <Fragment key={index}>
          <Heading size={"2"} as={"h4"}>
            {i.name}
          </Heading>
          <Space />
          <Identity identity={i} />
        </Fragment>
      ))}
    </Section>
  );
};

const Identity = ({
  identity: { contacts, licences, lifestyle, name, price, nuyens, quality },
}: {
  identity: SRIdentity;
}) => {
  const qualityStr = quality ? `i${quality}` : null;
  const lifestyleStr = lifestyle?.name || null;

  return (
    <>
      <MasonryGridNoSSR compact columns={2}>
        {(lifestyle || quality) && (
          <Container>
            <Card>
              <TitleMin
                inline
                subtitle={[lifestyleStr, qualityStr]
                  .filter((x) => x)
                  .join(" - ")}
              />
              {lifestyle?.description && (
                <ParagraphStandard>{lifestyle.description}</ParagraphStandard>
              )}
            </Card>
          </Container>
        )}
        {quality && (
          <Container>
            <Card title={"nuyens"}>
              <Flex justify={"between"} align={"end"} height={"9"}>
                <ParagraphStandard>_</ParagraphStandard>
                <ParagraphStandard>
                  {nuyens ? `${nuyens}¥` : null}
                </ParagraphStandard>
              </Flex>
            </Card>
          </Container>
        )}
        {Object.values(licences || {}).map((l) => (
          <Container key={l.name}>
            <Card title={"licence"}>
              <TitleMin inline subtitle={`${l.name} - i${l.quality}`} />
              {l.description && (
                <>
                  <Space />
                  <ParagraphStandard>{l.description}</ParagraphStandard>
                </>
              )}
            </Card>
          </Container>
        ))}
        {Object.values(contacts || {})?.map((c) => {
          return (
            <Container key={c.name}>
              <Card title={"contact"}>
                <TitleMin
                  title={c.name}
                  inline
                  subtitle={
                    <Box
                      style={{
                        display: "inline-block",
                      }}
                    >{`L${c.loyalty}-R${c.connection}`}</Box>
                  }
                />
                <Space />
                <ParagraphStandard>{c.description}</ParagraphStandard>
              </Card>
            </Container>
          );
        })}
      </MasonryGridNoSSR>
    </>
  );
};

const Container = ({
  children,
  width,
}: {
  children?: React.ReactNode;
  width?: string;
}) => {
  return (
    <Box
      pr={"2"}
      grow={"1"}
      pb={"2"}
      style={{
        width,
      }}
    >
      {children}
    </Box>
  );
};
