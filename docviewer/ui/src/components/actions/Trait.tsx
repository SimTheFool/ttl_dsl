import { SRTrait } from "@/app/SRDocument/character";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { capitalize } from "@/utils/capitalize";

type TraitProps = {
  trait: SRTrait;
  name: string;
};

export const Trait = ({ name, trait }: TraitProps) => {
  return (
    <Card
      style={{
        backgroundColor: "var(--gray-6)",
      }}
    >
      <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
      <Space />
      <ParagraphStandard>
        {<TextReplaced>{trait.description}</TextReplaced>}
      </ParagraphStandard>
    </Card>
  );
};
