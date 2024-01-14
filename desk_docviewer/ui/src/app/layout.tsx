"use client";

import { FileInput } from "@/components/controls/FileInput";
import { Menu } from "@/components/controls/Menu";
import { SelectInput } from "@/components/controls/Select";
import * as Form from "@radix-ui/react-form";
import "@radix-ui/themes/styles.css";
import { usePathname } from "next/navigation";
import "normalize.css/normalize.css";
import React, { useEffect } from "react";
import { FaEye, FaPrint } from "react-icons/fa";
import { FaFolder } from "react-icons/fa6";
import { ImInsertTemplate } from "react-icons/im";
import { LuSheet } from "react-icons/lu";
import { useLocalStorage } from "react-use";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/navigation";
import "./globals.css";
import { RenderingContextProvider } from "@/components/controls/RenderingContext";

const getTemplates = async (): Promise<string[]> => {
  return invoke("get_templates");
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const router = useRouter();
  const path = usePathname();
  const [open, setOpen] = React.useState(false);
  const [templates, setTemplates] = React.useState<string[]>([]);

  const [resolutionDir, setResolutionDir] =
    useLocalStorage<string>("resolutionDir");
  const [dataFile, setDataFile] = useLocalStorage<string>("dataFile");
  const [template, setTemplate] = useLocalStorage<string>("template");

  const isReadyToRender = resolutionDir && dataFile && template;
  const isRendering = path === `/${template}`;

  useEffect(() => {
    getTemplates().then((t) => setTemplates(t));
  }, [setTemplates]);

  return (
    <html lang="en">
      <RenderingContextProvider
        dataFile={dataFile}
        resolutionDir={resolutionDir}
        template={template}
      >
        <body
          style={{
            position: "relative",
          }}
        >
          <Menu isOpen={open} setOpen={setOpen}>
            <Form.Root
              style={{
                display: "flex",
                justifyContent: "space-between",
                width: "100%",
              }}
            >
              <span
                style={{
                  display: "flex",
                }}
              >
                <FileInput
                  name="resolutionDir"
                  value={resolutionDir}
                  onChange={setResolutionDir}
                  directory={true}
                >
                  <FaFolder />
                </FileInput>

                <FileInput
                  name="dataFile"
                  value={dataFile}
                  onChange={setDataFile}
                >
                  <LuSheet />
                </FileInput>

                <SelectInput
                  name="template"
                  value={
                    template && templates.includes(template)
                      ? template
                      : undefined
                  }
                  onChange={setTemplate}
                  options={templates}
                >
                  <ImInsertTemplate />
                </SelectInput>
              </span>

              <span
                style={{
                  display: "flex",
                }}
              >
                <button
                  type="button"
                  disabled={!isReadyToRender}
                  style={{
                    marginRight: "8px",
                    maxHeight: "25px",
                  }}
                  onClick={(e) => {
                    template && router.push(template);
                  }}
                >
                  <FaEye />
                </button>

                <button
                  type="button"
                  disabled={!isRendering}
                  onSubmit={(e) => {
                    e.preventDefault();
                  }}
                  style={{
                    marginRight: "8px",
                    maxHeight: "25px",
                  }}
                >
                  <FaPrint />
                </button>
              </span>

              <span />
            </Form.Root>
          </Menu>
          <main>{children}</main>
        </body>
      </RenderingContextProvider>
    </html>
  );
}
