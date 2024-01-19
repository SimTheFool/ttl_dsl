"use client";

import { useRenderingContext } from "@/components/controls/RenderingContext";
import { characters } from "@/mock/characters";
import { getData } from "@/utils/tauriAPI";
import { useAsync } from "react-use";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";
import { SRCharacter, parseCharacter } from "./character";

const char = characters.shrimp;

export default function Home() {
  const { dataFile, resolutionDir } = useRenderingContext();

  const { value } = useAsync(async () => {
    if (!dataFile || !resolutionDir) {
      return;
    }
    return await getData<SRCharacter>(dataFile, resolutionDir, parseCharacter);
  }, [dataFile, resolutionDir]);

  const images = value?.[1];
  const json = value?.[0];

  console.log("!!!!!!!!!!!", json);

  return (
    <>
      <Summary char={char} images={images} />
      <Inventory char={char} />
      <Powers char={char} />
      <Last char={char} images={images} />
    </>
  );
}
