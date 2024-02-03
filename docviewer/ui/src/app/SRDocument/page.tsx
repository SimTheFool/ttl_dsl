"use client";

import { RenderData } from "@/utils/tauriAPI";
import Inventory from "./Inventory";
import Powers from "./Powers";
import Summary from "./Summary";
import Last from "./Last";
import { parseCharacter } from "./character";

export default function Page() {
  return (
    <>
      <RenderData
        Child={({ data, images }) => (
          <>
            <Summary char={data} images={images} />
            <Inventory char={data} />
            <Powers char={data} />
            <Last images={images} />
          </>
        )}
        parser={parseCharacter}
      />
    </>
  );
}
