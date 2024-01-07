import { Flex, Box, Text } from "@radix-ui/themes";
import { MajorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { LoadAction as LoadActionType } from "@/mock/type";
import { Card } from "../Card";
import { Bullet } from "../Icons/Bullet";
import { Gauge } from "../Gauge";

type LoadActionProps = {
  action: LoadActionType;
};
export const LoadAction = ({ action }: LoadActionProps) => {
  return (
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin title={"Recharger"} />
          <Space />
          {action.description && (
            <ParagraphStandard>{action.description}</ParagraphStandard>
          )}
          <ParagraphStandard>
            <Gauge length={action.ammo} icon={<Bullet />} />
          </ParagraphStandard>
        </Box>
        <Box>
          <MajorAction />
        </Box>
      </Flex>
    </Card>
  );
};
