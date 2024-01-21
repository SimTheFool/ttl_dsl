import { Line } from "@/components/Line";
import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { PdfContainer } from "@/components/PdfContainer";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { Drone } from "@/components/items/Drone";
import { Item } from "@/components/items/Item";
import { Outfit } from "@/components/items/Outfit";
import { Slot } from "@/components/items/Slot";
import { Tech } from "@/components/items/Tech";
import { Weapon } from "@/components/items/Weapon";
import { getCharWeights } from "@/utils/getWeights";
import { Box } from "@radix-ui/themes";
import { SRCharacter } from "./character";

type Props = {
  char: SRCharacter;
};

export default function Inventory({ char }: Props) {
  const charWeights = getCharWeights(char);
  const pageWeight =
    charWeights.drones +
    charWeights.weapons +
    charWeights.outfits +
    charWeights.tech +
    charWeights.other;

  if (pageWeight < 30) {
    return (
      <PdfContainer>
        <AllInOne char={char} />
      </PdfContainer>
    );
  }

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
}

const AllInOne = ({ char }: { char: Character }) => {
  return (
    <>
      <BigObjects char={char} />
      <Space />
      <Space />
      <LittleObjects char={char} />
    </>
  );
};

const BigObjects = ({ char }: { char: Character }) => {
  return (
    <MasonryGridNoSSR columns={3}>
      <Box>
        <TitleSection>Inventaire</TitleSection>
        <Space />
      </Box>
      {Object.entries(char.drones || {}).map(([name, drone]) => {
        return (
          <Box pb={"2"} pr={"2"} key={name}>
            <Drone item={drone} name={name} />
          </Box>
        );
      })}
      {Object.entries(char.weapons || {}).map(([name, weapon]) => {
        return (
          <Box pb={"2"} pr={"2"} key={name}>
            <Weapon weapon={weapon} name={name} />
          </Box>
        );
      })}
      {Object.entries(char.outfits || {}).map(([name, outfit]) => {
        return (
          <Box pb={"2"} pr={"2"} key={name}>
            <Outfit outfit={outfit} name={name} />
          </Box>
        );
      })}
      {Object.entries(char.tech || {}).map(([name, tech]) => {
        return (
          <Box pb={"2"} pr={"2"} key={name}>
            <Tech tech={tech} name={name} />
          </Box>
        );
      })}
    </MasonryGridNoSSR>
  );
};

const LittleObjects = ({ char }: { char: Character }) => {
  return (
    <>
      <Box>
        <TitleSection>Consommables et outils</TitleSection>
        <Space />
      </Box>
      <MasonryGridNoSSR columns={4}>
        {Object.entries(char.other || {}).map(([name, item]) => {
          return (
            <Box pb={"4"} pr={"1"} key={name}>
              <Item item={item} name={name} />
            </Box>
          );
        })}
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
