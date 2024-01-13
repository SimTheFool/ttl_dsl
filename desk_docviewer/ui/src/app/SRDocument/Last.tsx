import Image from "next/image";
import { Box } from "@radix-ui/themes";
import { PdfContainer } from "@/components/PdfContainer";
import { Character } from "@/mock/type";
import { portraits } from "@/mock/characters";

type Props = {
  char: Character;
};
const portrait = portraits["shrimp"];

export default function Last({ char }: Props) {
  return (
    <PdfContainer>
      <Box
        style={{
          width: "100%",
          height: "100%",
        }}
      >
        {portrait && (
          <Image
            src={portrait}
            alt="character image"
            style={{
              objectFit: "contain",
              height: "100%",
              width: "auto",
            }}
          />
        )}
      </Box>
    </PdfContainer>
  );
}
