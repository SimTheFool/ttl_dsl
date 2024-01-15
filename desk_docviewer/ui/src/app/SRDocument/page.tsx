"use client";

import { useRenderingContext } from "@/components/controls/RenderingContext";
import { characters } from "@/mock/characters";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { useAsync } from "react-use";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";

const char = characters.shrimp;

const getData = async (dataFile: string, resolutionDir: string) => {
  const [json, images] = (await invoke("get_template_data", {
    dataFile: dataFile,
    resolutionDir: resolutionDir,
  })) as [json: unknown, images: Record<string, string>];

  const imagesWithAssetLinks = Object.fromEntries(
    Object.entries(images).map(([key, value]) => [key, convertFileSrc(value)])
  );

  return [json, imagesWithAssetLinks] as const;
};

export default function Home() {
  const { dataFile, resolutionDir } = useRenderingContext();

  const { value } = useAsync(async () => {
    if (!dataFile || !resolutionDir) {
      return;
    }
    return await getData(dataFile, resolutionDir);
  }, [dataFile, resolutionDir]);

  const images = value?.[1];
  const json = value?.[0];

  return (
    <>
      <Summary char={char} images={images} />
      <Inventory char={char} />
      <Powers char={char} />
      <Last char={char} images={images} />
    </>
  );
}
