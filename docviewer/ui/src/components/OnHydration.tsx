import { useEffect, useState } from "react";

type OnHydrationProps = {
  children: React.ReactNode;
};

export const OnHydration = ({ children }: OnHydrationProps) => {
  const [isHydrated, setIsHydrated] = useState(false);
  useEffect(() => {
    setIsHydrated(true);
  }, []);

  return <>{isHydrated ? children : null}</>;
};

export const useHydration = () => {
  const [isHydrated, setIsHydrated] = useState(false);
  useEffect(() => {
    setIsHydrated(true);
  }, []);
  return isHydrated;
};
