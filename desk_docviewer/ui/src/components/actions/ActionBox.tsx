import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { ReactElement, ReactNode } from "react";
import { Card } from "../Card";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import React from "react";
import { BaseAction } from "@/mock/type";
import { Maintained } from "../Icons/Maintained";
import { SpellDistance } from "../Icons/SpellDistance";
import { SpellNature } from "../Icons/SpellNature";
import { SpellZone } from "../Icons/SpellZone";
import { interleave } from "@/utils/interleave";
import { Damage } from "../Icons/Damage";

type ActionBoxProps = {
  title?: string;
  infos?: {
    maintained?: boolean;
  };
  children: {
    resources?: ReactElement;
    content?: ReactNode;
  };
};

export const ActionBox = ({
  children: { content, resources },
  infos: { maintained } = {},
  title,
}: ActionBoxProps) => {
  const resourcesChildren = React.Children.toArray(
    resources?.props.children
  ).filter((x) => x);

  const infosIcons = [
    nature && <SpellNature nature={nature} />,
    range && <SpellDistance range={range} />,
    zone && <SpellZone zone={zone} />,
    damage && (
      <>
        <span
          style={{
            fontWeight: "bold",
          }}
        >
          {damage}
        </span>
        <Damage />
      </>
    ),
  ].filter((x) => x) as React.JSX.Element[];

  return (
    <Card title={type}>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={<TextReplaced>{capitalize(title || "")}</TextReplaced>}
            subtitle={interleave(infosIcons, <Space inline />)}
            inline
          />
          <Space />
          {content}
        </Box>

        <Box pl={"1"}>
          {maintained && <Maintained />}
          {resourcesChildren?.length > 0 &&
            resourcesChildren.map((r, index) => (
              <Box pb={"1"} key={index}>
                {r}
              </Box>
            ))}
        </Box>
      </Flex>
    </Card>
  );
};
