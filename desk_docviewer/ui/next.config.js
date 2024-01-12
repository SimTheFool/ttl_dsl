/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  images: { unoptimized: true },
  assetPrefix: `${__dirname}/out`,
};

module.exports = nextConfig;
