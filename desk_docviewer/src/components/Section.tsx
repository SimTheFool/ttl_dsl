import {
  Box,
  Flex,
  Heading,
  Section as RadSection,
  Text,
} from "@radix-ui/themes";

type SectionProps = {
  children?: React.ReactNode;
  title?: React.ReactNode;
};

export const Section = ({ children, title }: SectionProps) => {
  return (
    <RadSection py={"0"} mb={"3"}>
      {title && (
        <Box pt={"0"} pb={"1"}>
          {title}
        </Box>
      )}
      {children}
    </RadSection>
  );
};
