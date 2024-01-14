"use client";

import { characters } from "@/mock/characters";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";
import { useRenderingContext } from "@/components/controls/RenderingContext";

type Props = {};

const char = characters.shrimp;

export default function Home({}: Props) {
  const { dataFile, resolutionDir } = useRenderingContext();
  return (
    <>
      <Summary char={char} />
      <Inventory char={char} />
      <Powers char={char} />
      <Last char={char} />
    </>
  );
}
