import shrimp from "./shrimp.json";
import shrimpPortrait from "./shrimp.png";

import type { Character } from "./type";

export const characters = {
  shrimp,
} as Record<string, Character>;

export const portraits = {
  shrimp: shrimpPortrait,
};
