"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

type Props = {};

export default function Home({}: Props) {
  const router = useRouter();
  useEffect(() => {
    router.push("/SRDocument");
  }, [router]);
  return <></>;
}
