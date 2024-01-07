import { PdfContainer } from "../../PdfContainer";
import Image from "next/image";
import { Box } from "@radix-ui/themes";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
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
