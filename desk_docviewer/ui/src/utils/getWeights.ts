import {
  BaseItem,
  Character,
  Companion,
  Drone,
  Outfit,
  Tech,
  Weapon,
} from "@/mock/type";

export const getCharWeights = (char: Character) => {
  const powers =
    Object.values(char.spells || {}).length +
    Object.values(char.rituals || {}).length +
    Object.values(char.complex_forms || {}).length;

  const companions =
    Object.values(char.other_companions || {}).reduce(
      (acc, c) => getCompanionWeight(c) + acc,
      0
    ) +
    Object.values(char.spirits || {}).reduce(
      (acc, c) => getCompanionWeight(c) + acc,
      0
    ) +
    Object.values(char.sprites || {}).reduce(
      (acc, c) => getCompanionWeight(c) + acc,
      0
    );

  const weapons = Object.values(char.weapons || {}).reduce(
    (acc, c) => getWeaponWeight(c) + acc,
    0
  );

  const outfits = Object.values(char.outfits || {}).reduce(
    (acc, c) => getOutfitWeight(c) + acc,
    0
  );

  const tech = Object.values(char.tech || {}).reduce(
    (acc, c) => getTechWeight(c) + acc,
    0
  );

  const other = Object.values(char.other || {}).reduce(
    (acc, c) => getOtherItemWeight(c) + acc,
    0
  );

  const drones = Object.values(char.drones || {}).reduce(
    (acc, c) => getDroneWeight(c) + acc,
    0
  );

  return {
    powers,
    companions,
    weapons,
    outfits,
    tech,
    other,
    drones,
  };
};

const getCompanionWeight = (companion: Companion) => {
  return (
    (companion.effects || []).length +
    Object.values(companion.actions || {}).length +
    (companion.description ? 1 : 0) +
    1
  );
};

const getWeaponWeight = (weapon: Weapon) => {
  return (
    (weapon.slots || []).length +
    Object.values(weapon.actions || {}).length +
    (weapon.description ? 1 : 0) +
    1
  );
};

const getOutfitWeight = (outfit: Outfit) => {
  return (
    (outfit.slots || []).length +
    Object.values(outfit.actions || {}).length +
    (outfit.description ? 1 : 0) +
    1
  );
};

const getTechWeight = (tech: Tech) => {
  return (tech.slots || []).length + (tech.description ? 1 : 0) + 1;
};

const getOtherItemWeight = (item: BaseItem) => {
  return (item.slots || []).length + (item.description ? 1 : 0) + 1;
};

const getDroneWeight = (drone: Drone) => {
  return (drone.slots || []).length + (drone.description ? 1 : 0) + 1;
};
