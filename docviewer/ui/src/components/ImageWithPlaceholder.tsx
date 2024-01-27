import Image from "next/image";

type ImageProps = Omit<Parameters<typeof Image>[0], "src"> & {
  src: string | undefined;
};

export const ImageWithPlaceholder = ({ src, alt, ...props }: ImageProps) => {
  return src ? (
    <Image src={src} alt={alt} {...props} />
  ) : (
    <div style={props.style}>{alt}</div>
  );
};
