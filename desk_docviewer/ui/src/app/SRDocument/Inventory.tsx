import { Line } from "@/components/Line";
import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { PdfContainer } from "@/components/PdfContainer";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { Object } from "@/components/items/Object";
import { Slot } from "@/components/items/Slot";
import { Box, Flex } from "@radix-ui/themes";
import { SRCharacter } from "./character";
import { ParagraphStandard } from "@/components/ParagraphStandard";

type Props = {
  char: SRCharacter;
};

const getObjectWeight = (obj: any) => {
  return (
    window.Object.values(obj.actions || {}).length +
    window.Object.values(obj.slots || {}).length
  );
};

export default function Inventory({ char }: Props) {
  const smallWeight = window.Object.values(char.small_inventory || {}).reduce(
    (w, obj) => {
      return w + getObjectWeight(obj);
    },
    0
  );

  const bigWeight = window.Object.values(char.big_inventory || {}).reduce(
    (w, obj) => {
      return w + getObjectWeight(obj);
    },
    0
  );

  if (smallWeight > 20 || bigWeight > 20)
    return (
      <>
        <PdfContainer>
          <BigObjects char={char} />
        </PdfContainer>
        <PdfContainer>
          <LittleObjects char={char} />
        </PdfContainer>
      </>
    );

  return (
    <PdfContainer>
      <AllInOne char={char} />
    </PdfContainer>
  );
}

const AllInOne = ({ char }: { char: SRCharacter }) => {
  return (
    <>
      <BigObjects char={char} />
      <Space />
      <Space />
      <LittleObjects char={char} />
    </>
  );
};

const BigObjects = ({ char }: { char: SRCharacter }) => {
  return (
    <MasonryGridNoSSR columns={3}>
      <Box>
        <TitleSection>Inventaire</TitleSection>
        <Space />
      </Box>
      {window.Object.entries(char.big_inventory || {}).map(([name, obj]) => {
        return (
          <Box pb={"2"} pr={"2"} key={name}>
            <Object object={obj} name={name} />
          </Box>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const LittleObjects = ({ char }: { char: SRCharacter }) => {
  return (
    <>
      <Box>
        <TitleSection>Consommables et outils</TitleSection>
        <Space />
      </Box>
      <MasonryGridNoSSR columns={3}>
        {window.Object.entries(char.small_inventory || {}).map(
          ([name, obj]) => {
            return (
              <Box pb={"2"} pr={"2"} key={name}>
                <Object object={obj} name={name} />
                <Flex pt={"1"}>
                  <ParagraphStandard>Restant:</ParagraphStandard>
                </Flex>
              </Box>
            );
          }
        )}
      </MasonryGridNoSSR>
      <Box px={"2"}>
        <Box>
          <TitleSection>Stockage de données</TitleSection>
          <Space />
        </Box>
        <Slot size="XL">
          <Line />
          <Line />
          <Line />
          <Line />
          <Line />
          <Line />
          <Line />
          <Line />
        </Slot>
      </Box>
    </>
  );
};
