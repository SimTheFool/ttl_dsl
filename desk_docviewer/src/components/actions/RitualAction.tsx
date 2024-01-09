import { capitalize } from "@/utils/capitalize";
import { Box } from "@radix-ui/themes";
import { Ritual } from "@/app/mock/type";
import { SpellNature } from "../Icons/SpellNature";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { ActionBox } from "./ActionBox";

type RitualActionProps = {
  name: string;
  action: Ritual;
};
export const RitualAction = ({
  name,
  action: { descriptions, type, duration, threshold, ...actionInfos },
}: RitualActionProps) => {
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
          <Box px={"1"}>
            <ParagraphStandard>{`${duration}`}</ParagraphStandard>
            <ParagraphStandard>{`|${threshold}|`}</ParagraphStandard>
          </Box>
        ),
      }}
    </ActionBox>
  );
};
