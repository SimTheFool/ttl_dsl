import { Heading } from "@radix-ui/themes";

type TitleSectionProps = {
  children: React.ReactNode;
};

export const TitleSection = ({ children }: TitleSectionProps) => {
  return (
    <Heading
      size={"4"}
      as={"h2"}
      style={{
        display: "block",
        textTransform: "uppercase",
      }}
    >
      {children}
    </Heading>
  );
};
