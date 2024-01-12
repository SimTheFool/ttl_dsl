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
        size={"1"}
        weight={"light"}
        style={{
          display: "inline-block",
        }}
      >
        <GiShotgun />
      </Text>
    </Box>
  );
};
