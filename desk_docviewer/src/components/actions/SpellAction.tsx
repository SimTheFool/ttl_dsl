import { capitalize } from "@/utils/capitalize";
import { Spell as SpellType } from "@/mock/type";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { ActionBox } from "./ActionBox";

type SpellActionProps = {
  name: string;
  action: SpellType;
};
export const SpellAction = ({
  name,
  action: { major, minor, descriptions, type, ...actionInfos },
}: SpellActionProps) => {
  return (
    <ActionBox title={name} infos={actionInfos} type={type}>
      {{
        content: Object.entries(descriptions || {}).map(
          ([key, description]) => (
            <ParagraphStandard key={key}>
              {key === "base" ? (
                <TextReplaced>{description}</TextReplaced>
              ) : (
                <>
                  <Space />
                  <Space />
                  <TitleMin
                    title={
                      <TextReplaced>{`${capitalize(key)}: `}</TextReplaced>
                    }
                    inline
                  />
                  <Space />
                  <TextReplaced>{description}</TextReplaced>
                </>
              )}
            </ParagraphStandard>
          )
        ),
        resources: (
          <>
            {Array.from({ length: major }).map((_, i) => (
              <MajorAction key={i} />
            ))}
            {Array.from({ length: minor }).map((_, i) => (
              <MinorAction key={i} />
            ))}
          </>
        ),
      }}
    </ActionBox>
  );
};
