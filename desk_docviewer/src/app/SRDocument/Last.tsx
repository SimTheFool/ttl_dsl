import Image from "next/image";
import { Box } from "@radix-ui/themes";
import { PdfContainer } from "@/components/PdfContainer";
import { Character } from "@/app/mock/type";

type Props = {
  char: Character;
};

export default function Last({ char }: Props) {
  //const portrait = portraits[name];

  return (
    <PdfContainer>
      <Box
        style={{
          width: "100%",
          height: "100%",
        }}
      >
        {/* {portrait && (
          <Image
            src={portrait}
            alt="character image"
            style={{
              objectFit: "contain",
              height: "100%",
              width: "auto",
            }}
          />
        )} */}
      </Box>
    </PdfContainer>
  );
}
