import { Interpreter } from "./bindgen/lib_ts";

export const getInterpreter = async () => {
  let interpreter = new Interpreter((str: string) => str);
  return interpreter;
};
