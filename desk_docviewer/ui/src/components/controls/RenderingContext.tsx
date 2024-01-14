"use client";

import React, { createContext } from "react";

type RenderingContext = {
  resolutionDir?: string;
  dataFile?: string;
  template?: string;
};
const renderingContext = createContext<RenderingContext>({});

export const RenderingContextProvider = ({
  children,
  resolutionDir,
  dataFile,
  template,
}: RenderingContext & { children: React.ReactNode }) => {
  return (
    <renderingContext.Provider
      value={{
        resolutionDir,
        dataFile,
        template,
      }}
    >
      {children}
    </renderingContext.Provider>
  );
};

export const useRenderingContext = () => React.useContext(renderingContext);
