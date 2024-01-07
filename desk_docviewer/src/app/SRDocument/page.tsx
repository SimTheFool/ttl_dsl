import { characters } from "@/mock/characters";
import { PdfBreak } from "../../components/PdfContainer";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summmary from "./Summary";

type Props = {};

const char = characters.shrimp;
//const portrait = portraits[name];

export default function Home({}: Props) {
  return (
    <>
      <Summmary char={char} />

      <PdfBreak />

      <Inventory char={char} />

      <PdfBreak />

      <Powers char={char} />

      <PdfBreak />

      <Last char={char} />
    </>
  );
}
