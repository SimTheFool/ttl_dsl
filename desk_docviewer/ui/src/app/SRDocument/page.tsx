"use client";

import { useRenderingContext } from "@/components/controls/RenderingContext";
import { getData } from "@/utils/tauriAPI";
import { useAsync } from "react-use";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";
import { SRCharacter, parseCharacter } from "./character";

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

  if (!json) {
    return null;
  }

  return (
    <>
      <Summary char={json} images={images} />
      <Inventory char={json} />
      <Powers char={json} />
      <Last images={images} />
    </>
  );
}
