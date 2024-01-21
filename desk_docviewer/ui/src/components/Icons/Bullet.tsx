import { Box, Text } from "@radix-ui/themes";
import { GiShotgun } from "react-icons/gi";

type BulletProps = {};
export const Bullet = ({}: BulletProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        transform: "translateY(1px)",
      }}
    >
      <Text
        weight={"light"}
        style={{
          display: "inline-block",
          fontSize: "10px",
        }}
      >
        <GiShotgun />
      </Text>
    </Box>
  );
};
