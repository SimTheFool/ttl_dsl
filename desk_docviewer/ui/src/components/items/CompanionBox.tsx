import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { ReactNode } from "react";
import { Companion as CompanionType } from "@/mock/type";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { SimpleAction } from "../actions/SimpleAction";
import styles from "./ItemCard.module.css";
import { Slot } from "./Slot";
import { Trait } from "../actions/Trait";
import { MasonryGridNoSSR } from "../MasonryGridNoSSR";

type CompanionBoxProps = {
  name: string;
  type?: string;
  companion: CompanionType;
  children?: ReactNode;
  noSlot?: boolean;
};

export const CompanionBox = ({
  name,
  type,
  companion,
  children,
  noSlot = false,
}: CompanionBoxProps) => {
  const actions = Object.entries(companion.actions || {}).map(
    ([name, action]) => (
      <SimpleAction name={name.toUpperCase()} action={action} key={name} />
    )
  );

  const effects = Object.entries(companion.effects || {}).map(
    ([name, effect]) => <Trait trait={effect} key={name} simple />
  );

  const skills = companion.skills && (
    <Card style={{ backgroundColor: "var(--gray-6)" }}>
      <TitleMin title={<TextReplaced>{"Compétences"}</TextReplaced>} />
      <ParagraphStandard>{companion.skills.join(" - ")}</ParagraphStandard>
    </Card>
  );

  const invokSlot = noSlot ? undefined : (
    <Slot size="M" note={"puissance - services - vie"}></Slot>
  );

  const bottomChildren = [skills, ...effects, ...actions, invokSlot];

  return (
    <Box>
      <Flex className={bottomChildren.length > 0 ? styles.noBorderBottom : ""}>
        <Card title={type}>
          <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
          <ParagraphStandard>
            {companion.description && (
              <>
                <Space />
                <TextReplaced>{companion.description}</TextReplaced>
              </>
            )}
            <Space />
            {children}
            <Space />
          </ParagraphStandard>
        </Card>
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
  type,
  companion,
  children,
  noSlot = false,
}: CompanionBoxProps) => {
  const actions = Object.entries(companion.actions || {}).map(
    ([name, action]) => <SimpleAction name={name} action={action} key={name} />
  );

  const effects = Object.entries(companion.effects || {}).map(
    ([name, effect]) => <Trait trait={effect} key={name} simple />
  );

  const skills = companion.skills && (
    <Card style={{ backgroundColor: "var(--gray-6)" }}>
      <TitleMin title={<TextReplaced>{"Compétences"}</TextReplaced>} />
      <ParagraphStandard>{companion.skills.join(" - ")}</ParagraphStandard>
    </Card>
  );

  const invokSlot = noSlot ? undefined : (
    <Slot size="INF" note={"pui. serv. vie"}></Slot>
  );

  const bottomChildren = [skills, ...effects, ...actions];

  return (
    <MasonryGridNoSSR compact columns={3}>
      <Flex p={"1"}>
        <Card title={type}>
          <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
          <ParagraphStandard>
            {companion.description && (
              <>
                <Space />
                <TextReplaced>{companion.description}</TextReplaced>
              </>
            )}
            <Space />
            {children}
            <Space />
          </ParagraphStandard>
        </Card>
        <Box
          grow={"1"}
          style={{
            width: "30%",
          }}
        >
          {invokSlot}
        </Box>
      </Flex>
      {bottomChildren.map((child, i) => (
        <Box key={i} p={"1"}>
          {child}
        </Box>
      ))}
    </MasonryGridNoSSR>
  );
};
