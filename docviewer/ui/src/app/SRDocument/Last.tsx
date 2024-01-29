import { ImageWithPlaceholder } from "@/components/ImageWithPlaceholder";
import { PdfContainer } from "@/components/PdfContainer";
import { Box } from "@radix-ui/themes";

type Props = {
  images?: Record<string, string>;
};

export default function Last({ images }: Props) {
  return (
    <PdfContainer noBreak>
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
