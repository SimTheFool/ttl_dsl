import { characters } from "@/mock/characters";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";

type Props = {};

const char = characters.shrimp;

export default function Home({}: Props) {
  return (
    <>
      <Summary char={char} />
      <Inventory char={char} />
      <Powers char={char} />
      <Last char={char} />
    </>
  );
}
