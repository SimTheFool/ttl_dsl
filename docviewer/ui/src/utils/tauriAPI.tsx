"use client";

import { useRenderingContext } from "@/components/controls/RenderingContext";
import { invoke } from "@tauri-apps/api/tauri";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

const getData = async <T,>(
  dataFile: string,
  resolutionDir: string,
  parser: (data: unknown) => T
) => {
  const [json, images] = (await invoke("get_template_data", {
    dataFile: dataFile,
    resolutionDir: resolutionDir,
  })) as [json: unknown, images: Record<string, string>];

  const imagesWithAssetLinks = Object.fromEntries(
    Object.entries(images).map(([key, value]) => [key, convertFileSrc(value)])
  );

  let parsedData: T;
  try {
    parsedData = parser(json);
  } catch (e) {
    console.error(`Parse data error: ${e}`);
    throw new Error(`Parse data error: ${e}`);
  }

  return [parsedData, imagesWithAssetLinks] as const;
};

type RenderDataProps<T> = {
  Child: React.FC<{ data: T; images: Record<string, string> }>;
  parser: (data: unknown) => T;
};
export const RenderData = <T,>({ Child, parser }: RenderDataProps<T>) => {
  const { dataFile, resolutionDir } = useRenderingContext();
  const [json, setJson] = useState<T>();
  const [images, setImages] = useState<Record<string, string>>({});

  useEffect(() => {
    if (!dataFile || !resolutionDir) {
      return;
    }
    getData(dataFile, resolutionDir, parser)
      .then(([json, images]) => {
        setJson(json);
        setImages(images);
      })
      .catch((e) => {
        console.error(e);
      });
  }, [dataFile, resolutionDir]);

  if (!json) {
    return null;
  }

  return <Child data={json} images={images} />;
};
