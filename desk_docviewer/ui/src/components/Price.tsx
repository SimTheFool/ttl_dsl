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
      {price}¥{unit && "/u"}
    </>
  );
};
