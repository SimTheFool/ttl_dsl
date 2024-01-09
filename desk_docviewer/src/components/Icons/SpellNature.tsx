import { Box } from "@radix-ui/themes";
import { PiWaveSquare } from "react-icons/pi";
import { Spell } from "@/app/mock/type";
import { BaseIcon } from "./BaseIcon";
import { PiWaveSine } from "react-icons/pi";

type SpellNatureProps = {
  nature: Spell["nature"];
};
export const SpellNature = ({ nature }: SpellNatureProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      {nature == "physique" && <Physical />}

      {nature == "mana" && <Mana />}

      {nature == "duale" && (
        <Box
          style={{
            display: "inline-block",
            position: "relative",
          }}
        >
          <Box
            style={{
              display: "inline-block",
              visibility: "hidden",
            }}
          >
            <Mana />
          </Box>

          <Box
            style={{
              display: "inline-block",
              position: "absolute",
              left: 0,
              transform: "scaleX(-1)",
            }}
          >
            <Mana />
          </Box>

          <Box
            style={{
              display: "inline-block",
              position: "absolute",
              left: 0,
            }}
          >
            <Physical />
          </Box>
        </Box>
      )}
    </Box>
  );
};

const Physical = () => {
  return (
    <BaseIcon size={14} inline>
      <PiWaveSquare
        style={{
          color: "black",
        }}
      />
    </BaseIcon>
  );
};

const Mana = () => {
  return (
    <BaseIcon size={14} inline>
      <PiWaveSine
        style={{
          color: "black",
        }}
      />
    </BaseIcon>
  );
};
