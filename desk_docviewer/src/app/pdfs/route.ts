import { pdfsConfig } from "@/utils/config";
import { NextResponse } from "next/server";
import { characters } from "@/mock/characters";

const getCharacterSheet = (charName: string) => [
  `./pdfs/${charName}/summary`,
  `./pdfs/${charName}/inventory`,
  `./pdfs/${charName}/powers`,
  /* ...(portraits[charName] ? [`./pdfs/${charName}/last`] : []), */
];

export async function GET() {
  const charNames = Object.keys(characters);

  const charSheetsKV = charNames.map((charName) => [
    charName,
    getCharacterSheet(charName),
  ]);

  return NextResponse.json({
    characters: {
      sheets: Object.fromEntries(charSheetsKV),
      metadata: {
        ...pdfsConfig,
      },
    },
  });
}
