import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";

export const getData = async <T>(
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
