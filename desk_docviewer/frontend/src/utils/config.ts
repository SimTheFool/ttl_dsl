import { getA4FormatFromWidth } from "./a4format";

export const pdfsConfig = {
  size: { ...getA4FormatFromWidth(788) },
};
