"use client";

import { RenderData } from "@/utils/tauriAPI";
import Inventory from "./Inventory";
import Last from "./Last";
import Powers from "./Powers";
import Summary from "./Summary";
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
          </>
        )}
        parser={parseCharacter}
      />
    </>
  );
}
