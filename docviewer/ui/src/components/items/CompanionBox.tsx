import { SRCompanion } from "@/app/SRDocument/character";
import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { ReactNode } from "react";
import { Card } from "../Card";
import { MasonryGridNoSSR } from "../MasonryGridNoSSR";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { SimpleAction } from "../actions/SimpleAction";
import { Trait } from "../actions/Trait";
import styles from "./ItemCard.module.css";

type CompanionBoxProps = {
  name: string;
  companion: SRCompanion;
  children?: ReactNode;
  aside?: ReactNode;
  sub?: ReactNode;
};

export const CompanionBox = ({
  name,
  companion,
  children,
  aside,
  sub,
}: CompanionBoxProps) => {
  const actions = Object.entries(companion.actions || {}).map(
    ([name, action]) => (
      <SimpleAction name={capitalize(name)} action={action} key={name} />
    )
  );

  const traits = Object.entries(companion.traits || {}).map(([name, trait]) => (
    <Trait trait={trait} key={name} name={name} />
  ));

  const skills = companion.skills && (
    <Card style={{ backgroundColor: "var(--gray-6)" }}>
      <TitleMin title={<TextReplaced>{"Compétences"}</TextReplaced>} />
      <ParagraphStandard>
        {Object.values(companion.skills).join(" - ")}
      </ParagraphStandard>
    </Card>
  );

  const bottomChildren = [skills, ...traits, ...actions, aside].filter(
    (x) => x
  );

  return (
    <Box>
      <Flex className={bottomChildren.length > 0 ? styles.noBorderBottom : ""}>
        <Card>
          <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
          <Space />
          <ParagraphStandard>
            {children}
            <Space />
          </ParagraphStandard>
        </Card>
        {sub}
      </Flex>

      <MasonryGridNoSSR compact columns={1}>
        {bottomChildren.map((child, i) => (
          <Box key={i} className={i == 0 ? "" : styles.bottom}>
            {child}
          </Box>
        ))}
      </MasonryGridNoSSR>
      <Space />
    </Box>
  );
};

export const ErgoCompanionBox = ({
  name,
  companion,
  children,
  aside,
}: CompanionBoxProps) => {
  const actions = Object.entries(companion.actions || {}).map(
    ([name, action]) => <SimpleAction name={name} action={action} key={name} />
  );

  const effects = Object.entries(companion.traits || {}).map(
    ([name, trait]) => <Trait trait={trait} name={name} key={name} />
  );

  const skills = companion.skills && (
    <Card style={{ backgroundColor: "var(--gray-6)" }}>
      <TitleMin title={<TextReplaced>{"Compétences"}</TextReplaced>} />
      <ParagraphStandard>
        {Object.values(companion.skills).join(" - ")}
      </ParagraphStandard>
    </Card>
  );

  const bottomChildren = [skills, ...effects, ...actions];

  return (
    <MasonryGridNoSSR compact columns={3}>
      <Box>
        <Card>
          <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
          <Space />
          <ParagraphStandard>{children}</ParagraphStandard>
        </Card>
        {aside}
      </Box>
      {bottomChildren.map((child, i) => (
        <Box key={i} p={"1"}>
          {child}
        </Box>
      ))}
    </MasonryGridNoSSR>
  );
};

const HeadCard = ({ name, children }: any) => {};
