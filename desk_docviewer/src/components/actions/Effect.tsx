import { Effect as EffectType } from "@/app/mock/type";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";

type EffectProps = {
  effect: EffectType;
  simple?: boolean;
};

export const Effect = ({ effect, simple = false }: EffectProps) => {
  return (
    <Card
      title={!simple && effect.type}
      style={{
        backgroundColor: "var(--gray-6)",
      }}
    >
      <TitleMin title={effect.name} />
      <Space />
      <ParagraphStandard>
        {effect.description && (
          <TextReplaced>{effect.description}</TextReplaced>
        )}
      </ParagraphStandard>
    </Card>
  );
};
