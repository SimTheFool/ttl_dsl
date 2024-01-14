"use client";

import { characters } from "@/mock/characters";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";
import { useRenderingContext } from "@/components/controls/RenderingContext";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";

type Props = {};

const char = characters.shrimp;

const getData = async (
  dataFile: string,
  resolutionDir: string
): Promise<string[]> => {
  return invoke("get_json_data", {
    dataFile: dataFile,
    resolutionDir: resolutionDir,
  });
};

export default function Home({}: Props) {
  const { dataFile, resolutionDir } = useRenderingContext();

  useEffect(() => {
    dataFile &&
      resolutionDir &&
      getData(dataFile, resolutionDir).then((d) => console.log("########", d));
  }, [dataFile, resolutionDir]);

  return (
    <>
      <Summary char={char} />
      <Inventory char={char} />
      <Powers char={char} />
      <Last char={char} />
    </>
  );
}
