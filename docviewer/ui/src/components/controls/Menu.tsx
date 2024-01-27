import * as Collapsible from "@radix-ui/react-collapsible";
import { Box } from "@radix-ui/themes";
import { FaCog } from "react-icons/fa";

type MenuProps = {
  isOpen: boolean;
  setOpen: (x: boolean) => void;
  children?: React.ReactNode;
};

export const Menu = ({ isOpen, setOpen, children }: MenuProps) => {
  return (
    <Collapsible.Root
      className="menu"
      open={isOpen}
      onOpenChange={setOpen}
      style={{
        position: "fixed",
        top: 0,
        right: 0,
        zIndex: 99,
        display: "flex",
        flexDirection: "row-reverse",
        width: "100%",
      }}
    >
      <Collapsible.Trigger asChild>
        <button
          style={{
            height: "36px",
            width: "36px",
            zIndex: 100,
            cursor: "pointer",
          }}
        >
          <FaCog />
        </button>
      </Collapsible.Trigger>

      <Collapsible.Content asChild>
        <Box
          style={{
            position: "absolute",
            top: 0,
            paddingRight: "16px",
            width: "100%",
            backgroundColor: "whitesmoke",
            border: "1px solid lightgray",
            height: isOpen ? "50px" : "0px",
            display: "flex",
            alignItems: "center",
            alignContent: "center",
          }}
        >
          {children}
        </Box>
      </Collapsible.Content>
    </Collapsible.Root>
  );
};
