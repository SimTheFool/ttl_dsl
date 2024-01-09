import { PiDiamondLight } from "react-icons/pi";
import { BaseAction1 } from "@/app/mock/type";
import { Gauge } from "../Gauge";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Ruler } from "../Ruler";
import { TextReplaced } from "../Text";
import { ActionBox } from "./ActionBox";

type BaseActionProps = {
  name: string;
  action: BaseAction1;
  type?: string;
};
export const SimpleAction = ({
  name,
  type,
  action: { major, minor, description, gauge, score, ...actionInfos },
}: BaseActionProps) => {
  return (
    <ActionBox title={name} infos={actionInfos} type={type}>
      {{
        content: (
          <>
            {gauge && <Gauge length={gauge} icon={<PiDiamondLight />} />}
            {description && (
              <ParagraphStandard>
                <TextReplaced>{description}</TextReplaced>
              </ParagraphStandard>
            )}
            {score != undefined && (
              <ParagraphStandard>
                <Ruler grade={[score]} inter={[score]} />
              </ParagraphStandard>
            )}
          </>
        ),
        resources: (
          <>
            {major &&
              Array.from({ length: major }).map((_, i) => (
                <MajorAction key={i} />
              ))}
            {minor &&
              Array.from({ length: minor }).map((_, i) => (
                <MinorAction key={i} />
              ))}
          </>
        ),
      }}
    </ActionBox>
  );
};
