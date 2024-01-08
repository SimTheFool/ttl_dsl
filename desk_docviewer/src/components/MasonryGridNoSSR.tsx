"use client";

import dynamic from "next/dynamic";

export const MasonryGridNoSSR = dynamic(() => import("./MasonryGrid"), {
  ssr: false,
});
