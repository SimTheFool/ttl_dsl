import Image from "next/image";
import { Box } from "@radix-ui/themes";
import { PdfContainer } from "@/components/PdfContainer";
import { Character } from "@/mock/type";
import { portraits } from "@/mock/characters";
import { ImageWithPlaceholder } from "@/components/ImageWithPlaceholder";

type Props = {
  char: Character;
  images?: Record<string, string>;
};

export default function Last({ char, images }: Props) {
  return (
    <PdfContainer>
      <Box
        style={{
          width: "100%",
          height: "100%",
        }}
      >
        {images?.portrait && (
          <ImageWithPlaceholder
            src={images?.portrait}
            alt="portrait"
            width={100}
            height={200}
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
