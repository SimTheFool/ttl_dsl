import { capitalize } from "@/utils/capitalize";
import { BaseItem } from "@/app/mock/type";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Price } from "../Price";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { TextReplaced } from "../Text";
import { Box, Flex } from "@radix-ui/themes";
import styles from "./ItemCard.module.css";
import React from "react";
import { Slot } from "./Slot";
import { Hand } from "../Icons/Hand";
import { MasonryGridNoSSR } from "../MasonryGridNoSSR";

type ItemCardProps = {
  children?: {
    bottom?: React.ReactElement;
    inner?: React.ReactNode;
  };
  item: BaseItem;
  name: string;
  noHand?: boolean;
};

export const ItemCard = ({
  item,
  name,
  children,
  noHand = false,
}: ItemCardProps) => {
  const bottomChildren = React.Children.toArray(
    children?.bottom?.props.children
  ).filter((x) => x);

  const bottomChildrenWithSlots = [
    ...bottomChildren,
    ...(item.slots || []).map((slot) => {
      return (
        <Slot
          size={slot.size}
          key={slot.name}
          concealment={slot.concealment}
          note={slot.name}
        ></Slot>
      );
    }),
  ];

  const bottomItemNb = bottomChildrenWithSlots.length;

  return (
    <Box>
      <Flex className={bottomItemNb > 0 ? styles.noBorderBottom : ""}>
        <Card>
          <Flex justify={"between"}>
            <Box>
              <TitleMin
                inline
                title={
                  <TextReplaced>
                    {`${name.toUpperCase()} ${
                      item.quantity ? `x ${item.quantity}` : ""
                    }`}
                  </TextReplaced>
                }
                subtitle={item.manufacturer}
              />
              <TitleMin
                subtitle={
                  <>
                    <Price
                      price={item.price}
                      unit={
                        item.quantity != undefined && item.quantity > 1
                          ? true
                          : false
                      }
                    />
                    <Space inline />
                    {!item.legal ? "illégal" : item.licenced ? "licencié" : ""}
                  </>
                }
              />
            </Box>

            {!noHand && (
              <Box pl={"1"}>
                <Hand n={item.concealment} />
              </Box>
            )}
          </Flex>

          <ParagraphStandard>
            {item.description && (
              <>
                <Space />
                <TextReplaced>{item.description}</TextReplaced>
              </>
            )}
            <Space />
            {children?.inner}
          </ParagraphStandard>
        </Card>
      </Flex>
      <MasonryGridNoSSR compact columns={1}>
        {bottomChildrenWithSlots.map((child, i) => (
          <Box key={i} className={i == 0 ? "" : styles.bottom}>
            {child}
          </Box>
        ))}
      </MasonryGridNoSSR>
    </Box>
  );
};
