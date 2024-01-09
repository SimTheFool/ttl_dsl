import { Flex } from "@radix-ui/themes";
import { BaseItem } from "@/app/mock/type";
import { ParagraphStandard } from "../ParagraphStandard";
import { ItemCard } from "./ItemCard";

type ItemProps = { item: BaseItem; name: string };

export const Item = ({ item, name }: ItemProps) => {
  return (
    <>
      <ItemCard item={item} name={name} noHand />
      <Flex pt={"1"}>
        <ParagraphStandard>Restant:</ParagraphStandard>
      </Flex>
    </>
  );
};
