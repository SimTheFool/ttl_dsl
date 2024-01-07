import { Space } from "./Space";

export const Price = ({
  price,
  unit = false,
}: {
  price: number;
  unit?: boolean;
}) => {
  return (
    <>
      {price}
      <Space inline />¥{unit && "/u"}
    </>
  );
};
