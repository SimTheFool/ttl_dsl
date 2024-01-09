import { Card } from "@/components/Card";
import { Section } from "@/components/Section";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { TitleMin } from "@/components/TitleMin";
import { Box, Flex } from "@radix-ui/themes";
import { Character, Identity as CharIdentity } from "@/mock/type";
import { capitalize } from "@/utils/capitalize";
import { TitleSection } from "@/components/TitleSection";
import { Space } from "@/components/Space";
import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { Fragment } from "react";

type IdentitiesProps = {
  char: Character;
};

export const Identities = ({ char }: IdentitiesProps) => {
  return (
    <Section title={<TitleSection>Identités</TitleSection>}>
      {char.identities?.map((i) => (
        <Fragment key={i.name}>
          <Identity identity={i} />
        </Fragment>
      ))}
    </Section>
  );
};

const Identity = ({
  identity: {
    contacts,
    description,
    licences,
    lifestyle,
    name,
    nuyens,
    price,
    quality,
  },
}: {
  identity: CharIdentity;
}) => {
  const qualityStr = quality ? `${quality} - ${price}¥` : null;
  const lifestyleStr = lifestyle
    ? `${lifestyle?.name} - ${lifestyle?.price}¥`
    : null;

  return (
    <>
      {(lifestyle || quality) && (
        <Container>
          <Card>
            <Box>
              <TitleMin
                title={name && capitalize(name)}
                subtitle={[qualityStr, lifestyleStr]
                  .filter((x) => x)
                  .join(" - ")}
              />
              <ParagraphStandard>{description}</ParagraphStandard>
            </Box>
          </Card>
        </Container>
      )}
      <MasonryGridNoSSR compact columns={2}>
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
        {licences?.map((l) => (
          <Container key={l.name}>
            <Card title={"licence"}>
              <TitleMin title={l.name} subtitle={`${l.quality}-${l.price}¥`} />
              <Space />
              <ParagraphStandard>{l.description}</ParagraphStandard>
            </Card>
          </Container>
        ))}
        {contacts?.map((c) => {
          return (
            <Container key={c.name}>
              <Card title={"contact"}>
                <TitleMin
                  title={c.name}
                  subtitle={`L${c.loyalty}-R${c.connection}`}
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
