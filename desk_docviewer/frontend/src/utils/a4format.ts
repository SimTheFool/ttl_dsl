export const getA4FormatFromWidth = (width: number) => {
  const ratio = 1.414;
  const height = width * ratio;
  return { width: Math.floor(width), height: Math.floor(height) };
};
