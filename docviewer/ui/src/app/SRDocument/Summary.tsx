import { Header } from "@/components/Header";
import { ImageWithPlaceholder } from "@/components/ImageWithPlaceholder";
import { PdfContainer } from "@/components/PdfContainer";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Box, Grid } from "@radix-ui/themes";
import { Identities } from "./_Indentities";
import { Monitors } from "./_Monitors";
import { Resources } from "./_Resources";
import { Skills } from "./_Skills";
import { Stats } from "./_Stats";
import { Traits } from "./_Traits";
import { SRCharacter } from "./character";

type Props = {
  char: SRCharacter;
  images?: Record<string, string>;
};

export default function Summary({ char, images }: Props) {
  return (
    <PdfContainer>
      <Box
        style={{
          display: "flex",
          flexDirection: "column",
          height: "100%",
        }}
      >
        <Header char={char} />
        <Grid
          columns="2"
          gap="2"
          style={{
            gridTemplateColumns: "58% 42%",
            gridTemplateRows: "1fr",
          }}
          pt={"1"}
          px={"2"}
        >
          <Box>
            <Stats char={char} />
            <Resources char={char} />
            <Monitors char={char} />
          </Box>

          <Box>
            <Box
              pl={"2"}
              style={{
                borderLeft: "2px solid var(--gray-10)",
              }}
            >
              <Identities char={char} />
            </Box>
            <Box
              pl={"2"}
              style={{
                borderLeft: "2px solid var(--gray-10)",
              }}
            >
              <Skills char={char} />
            </Box>
          </Box>
        </Grid>
        <Box
          pt={"3"}
          mx={"3"}
          style={{
            borderTop: "2px solid var(--gray-10)",
          }}
        >
          <Traits char={char} />
        </Box>

        <Box
          grow={"1"}
          pt={"3"}
          mx={"3"}
          style={{
            borderTop: "2px solid var(--gray-10)",
            height: "0px",
            position: "relative",
          }}
        >
          <ImageWithPlaceholder
            src={images?.portrait}
            width={100}
            height={200}
            alt="portrait"
            style={{
              opacity: 0.7,
              position: "absolute",
              right: 0,
              top: 0,
              objectFit: "contain",
              height: "100%",
              width: "auto",
              marginLeft: "auto",
            }}
          />
          <Section title={<TitleSection>Notes</TitleSection>}>
            {Array.from({ length: 15 }).map((_, index) => (
              <Box
                key={index}
                style={{
                  width: "100%",
                  height: "calc(25px * var(--scaling)",
                  borderBottom: "1px solid var(--gray-8)",
                }}
              />
            ))}
          </Section>
        </Box>
      </Box>
    </PdfContainer>
  );
}
