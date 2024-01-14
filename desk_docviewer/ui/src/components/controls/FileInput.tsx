import * as Form from "@radix-ui/react-form";
import { DialogFilter, open as openDialogTauri } from "@tauri-apps/api/dialog";

type FileInputProps = {
  name: string;
  value: string | undefined;
  onChange: (x: string) => void;
  children?: React.ReactNode;
  title?: string;
  filters?: DialogFilter[];
  multiple?: boolean;
  directory?: boolean;
  recursive?: boolean;
};

export const FileInput = ({
  name,
  value,
  onChange,
  children,
  ...dialogParams
}: FileInputProps) => {
  return (
    <Form.Field
      name={name}
      style={{
        display: "flex",
        alignContent: "baseline",
        alignItems: "baseline",
      }}
    >
      <Form.Label
        style={{
          marginLeft: "8px",
          marginRight: "8px",
        }}
      >
        {children}
      </Form.Label>
      <span
        style={{
          display: "inline-block",
          width: "150px",
        }}
      >
        <Form.Control asChild>
          <input
            className="Input"
            type="string"
            required
            style={{
              width: "100%",
              cursor: "pointer",
            }}
            readOnly
            title={value}
            value={value}
            onClick={async () => {
              let newValue = await openDialogTauri({
                ...dialogParams,
                defaultPath: value,
              });
              if (typeof newValue == "string") {
                onChange(newValue);
              }
            }}
          />
        </Form.Control>
        <Form.Message
          match="valueMissing"
          style={{
            display: "block",
            maxWidth: "100%",
            color: "red",
          }}
        >
          Champs requis
        </Form.Message>
      </span>
    </Form.Field>
  );
};
