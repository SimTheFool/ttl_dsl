import { Box } from "@radix-ui/themes";

type RulerProps = {
  grade: (string | number)[];
  inter?: (string | number)[];
};

export const Ruler = ({ grade, inter }: RulerProps) => {
  return (
    <Box
      style={{
        borderBottom: "1px solid black",
        display: "inline-block",
      }}
    >
      {grade.map((nb, i) => (
        <Box
          key={i}
          pl={i == 0 ? "2" : "3"}
          style={{
            display: "inline-block",
            position: "relative",
          }}
        >
          <Box
            style={{
              visibility: inter ? "hidden" : "visible",
            }}
          >
            {nb}m
          </Box>
          {inter && (
            <Box
              style={{
                position: "absolute",
                bottom: 0,
                left: "20%",
              }}
            >
              {inter?.[i]}
            </Box>
          )}

          <Box
            pb={"1"}
            style={{
              borderRight: "1px solid black",
            }}
          />
        </Box>
      ))}
    </Box>
  );
};
