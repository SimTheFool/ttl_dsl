import { MasonryGridNoSSR } from "@/components/MasonryGridNoSSR";
import { Monitor } from "@/components/Monitor";
import { Box } from "@radix-ui/themes";
import { SRCharacter } from "./character";

type MonitorsProps = {
  char: SRCharacter;
};

export const Monitors = ({ char }: MonitorsProps) => {
  return (
    <>
      <Box pt={"2"}>
        <MasonryGridNoSSR columns={2}>
          <Monitor
            columns={4}
            hit={char.stats.hit_phy}
            title={"Dom. Physique"}
          />
          <Monitor
            columns={4}
            hit={char.stats.hit_stun}
            title={"Dom. Etourdissant"}
          />
          <Monitor columns={6} hit={char.stats.hit_over} title={"Surplus"} />
        </MasonryGridNoSSR>
      </Box>
    </>
  );
};
