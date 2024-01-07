import { MasonryGrid } from "@/components/MasonryGrid";
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
import { characters } from "@/mock/characters";
import { Character } from "@/mock/type";
import { PdfBreak, PdfContainer } from "../../PdfContainer";
import { Line } from "@/components/Line";

type Props = {
  params: {
    name: string;
  };
};

export default async function Page({ params: { name } }: Props) {
  const char = characters[name];
  const charWeights = getCharWeights(char);
  const pageWeight =
    charWeights.drones +
    charWeights.weapons +
    charWeights.outfits +
    charWeights.tech +
    charWeights.other;

  if (pageWeight < 30) {
    return (
      <PdfContainer footer={"INVENTAIRE"}>
        <AllInOne char={char} />
      </PdfContainer>
    );
  }

  return (
    <>
      <PdfContainer footer={"INVENTAIRE"}>
        <BigObjects char={char} />
      </PdfContainer>
      <PdfBreak />
      <PdfContainer footer={"CONSOMMABLES"}>
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
    <MasonryGrid columns={3}>
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
    </MasonryGrid>
  );
};

const LittleObjects = ({ char }: { char: Character }) => {
  return (
    <>
      <Box>
        <TitleSection>Consommables et outils</TitleSection>
        <Space />
      </Box>
      <MasonryGrid columns={4}>
        {Object.entries(char.other || {}).map(([name, item]) => {
          return (
            <Box pb={"4"} pr={"1"} key={name}>
              <Item item={item} name={name} />
            </Box>
          );
        })}
      </MasonryGrid>
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
