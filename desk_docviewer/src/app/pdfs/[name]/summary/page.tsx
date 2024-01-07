import { Effects } from "@/app/_Effects";
import { Identities } from "@/app/_Indentities";
import { Monitors } from "@/app/_Monitors";
import { Resources } from "@/app/_Resources";
import { Skills } from "@/app/_Skills";
import { Stats } from "@/app/_Stats";
import { Header } from "@/components/Header";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Box, Grid } from "@radix-ui/themes";
import { PdfContainer } from "../../PdfContainer";
import { characters } from "@/mock/characters";
import { Character } from "@/mock/type";
import Image from "next/image";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
  const char = characters[name];
  //const portrait = portraits[name];

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
          <Effects char={char} />
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
          {/* {portrait && (
            <Image
              src={portrait}
              alt="character image"
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
          )} */}
          <Section title={<TitleSection>Notes</TitleSection>}>
            {Array.from({ length: 15 }).map(() => (
              <Box
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
