import * as Form from "@radix-ui/react-form";

type SelectInputProps = {
  name: string;
  options: string[];
  value: string | undefined;
  onChange: (x: string) => void;
  children?: React.ReactNode;
};

export const SelectInput = ({
  name,
  value,
  onChange,
  children,
  options,
}: SelectInputProps) => {
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
          <select
            required
            value={value}
            style={{
              width: "100%",
            }}
            onChange={(e) => onChange(e.target.value)}
          >
            <option></option>
            {options.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </Form.Control>

        <Form.Message
          match="valueMissing"
          style={{
            display: "block",
            maxWidth: "100%",
            color: "red",
          }}
        >
          Champ requis
        </Form.Message>
      </span>
    </Form.Field>
  );
};
