import { ZodType, z } from "zod";

export type SRCharacter = z.infer<typeof characterParser>;

const asLeaf = <T extends ZodType<any, any, any>>(v: T) =>
  z.object({
    value: v,
    metas: z.array(z.string()).optional(),
  });

const asPseudoArray = <T extends ZodType<any, any, any>>(v: T) =>
  z.record(z.string(), v);

const identityParser = z.object({
  name: asLeaf(z.string()).optional(),
  price: asLeaf(z.number()).optional(),
  quality: asLeaf(z.number()).optional(),
  contacts: asPseudoArray(
    z.object({
      name: asLeaf(z.string()),
      loyalty: asLeaf(z.number()),
      connection: asLeaf(z.number()),
      description: asLeaf(z.string()),
    })
  ).optional(),
  lifestyles: asPseudoArray(
    z.object({
      name: asLeaf(z.string()),
      description: asLeaf(z.string()).optional(),
    })
  ).optional(),
  licences: asPseudoArray(
    z.object({
      name: asLeaf(z.string()),
      description: asLeaf(z.string()).optional(),
      price: asLeaf(z.number()),
      quality: asLeaf(z.number()),
    })
  ).optional(),
});

const skillParser = z.object({
  score: asLeaf(z.number()).optional(),
  specializations: asPseudoArray(asLeaf(z.string())).optional(),
  masterizations: asPseudoArray(asLeaf(z.string())).optional(),
});

const traitParser = z.object({
  description: asLeaf(z.string()),
});

const statsParser = z.object({
  con: asLeaf(z.number()),
  con_mod: asLeaf(z.number()),
  agi: asLeaf(z.number()),
  agi_mod: asLeaf(z.number()),
  rea: asLeaf(z.number()),
  rea_mod: asLeaf(z.number()),
  for: asLeaf(z.number()),
  for_mod: asLeaf(z.number()),
  vol: asLeaf(z.number()),
  vol_mod: asLeaf(z.number()),
  log: asLeaf(z.number()),
  log_mod: asLeaf(z.number()),
  int: asLeaf(z.number()),
  int_mod: asLeaf(z.number()),
  cha: asLeaf(z.number()),
  cha_mod: asLeaf(z.number()),
  ess: asLeaf(z.number()),
  edge: asLeaf(z.number()),
  resist_phy: asLeaf(z.number()),
  resist_ment: asLeaf(z.number()),
  def_phy: asLeaf(z.number()),
  def_ment: asLeaf(z.number()),
  init_dice: asLeaf(z.number()),
  init_base: asLeaf(z.number()),
  action_maj: asLeaf(z.number()),
  action_min: asLeaf(z.number()),
  hit_phy: asLeaf(z.number()),
  hit_stun: asLeaf(z.number()),
  hit_over: asLeaf(z.number()),
  heal: asLeaf(z.number()),
  res: asLeaf(z.number()),
  submersion: asLeaf(z.number()),
  resist_drain: asLeaf(z.number()),
  firewall: asLeaf(z.number()),
  traitement: asLeaf(z.number()),
  corruption: asLeaf(z.number()),
  attaque: asLeaf(z.number()),
});

const characterParser = z.object({
  name: asLeaf(z.string()),
  tags: asPseudoArray(asLeaf(z.string())),
  knowledges: asPseudoArray(asLeaf(z.string())),
  identities: asPseudoArray(identityParser),
  stats: statsParser,
  skills: z.record(z.string(), skillParser),
  traits: z.record(z.string(), traitParser),
});

export const parseCharacter: (data: unknown) => SRCharacter =
  characterParser.parse;
