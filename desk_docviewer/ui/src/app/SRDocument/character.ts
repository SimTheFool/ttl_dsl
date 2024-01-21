import { ZodType, z } from "zod";

export type SRCharacter = z.infer<typeof characterParser>;
export type SRIdnetity = z.infer<typeof identityParser>;
export type SRTrait = z.infer<typeof traitParser>;
export type SRAction = z.infer<typeof actionParser>;
export type SRCompanion = z.infer<typeof companionParser>;

const asLeaf = <T extends ZodType<any, any, any>>(v: T) =>
  z.object({
    value: v,
    metas: z.array(z.string()).optional(),
  });

const asPseudoArray = <T extends ZodType<any, any, any>>(v: T) =>
  z.record(z.string(), v);

const identityParser = z.object({
  name: z.string().optional(),
  price: z.number().optional(),
  nuyens: z.number().optional(),
  quality: z.number().optional(),
  contacts: asPseudoArray(
    z.object({
      name: z.string(),
      loyalty: z.number(),
      connection: z.number(),
      description: z.string(),
    })
  ).optional(),
  lifestyle: z
    .object({
      name: z.string(),
      description: z.string().optional(),
      price: z.number(),
    })
    .optional(),
  licences: asPseudoArray(
    z.object({
      name: z.string(),
      description: z.string().optional(),
      price: z.number(),
      quality: z.number(),
    })
  ).optional(),
});

const skillParser = z.object({
  score: z.number().optional(),
  specializations: asPseudoArray(z.string()).optional(),
  masterizations: asPseudoArray(z.string()).optional(),
});

const traitParser = z.object({
  description: z.string(),
});

const actionParser = z.object({
  test: z.string().optional(),
  major: z.number().optional(),
  minor: z.number().optional(),
  maintained: z.boolean().optional(),
  reaction: z.boolean().optional(),
  description: z.string().optional(),
  damage: z.number().optional(),
  ammo: z.number().optional(),
  gauge: z.number().optional(),
  ranges: z
    .object({
      contact: z.number(),
      near: z.number(),
      short: z.number(),
      mid: z.number(),
      far: z.number(),
    })
    .optional(),
});

const companionParser = z.object({
  name: z.string(),
  stats_primary: z
    .object({
      major: z.number(),
      minor: z.number(),
      hit: z.number().optional(),
      hit_formula: z.string().optional(),
    })
    .optional(),
  stats_secondary: z.record(z.string(), z.number()).optional(),
  skills: asPseudoArray(z.string()).optional(),
  traits: asPseudoArray(traitParser).optional(),
  actions: z.record(z.string(), actionParser).optional(),
});

const slotParser = z.object({
  name: z.string().optional(),
  size: z.enum(["S", "M", "L", "XL"]),
  concealment: z.number().optional(),
});

const objectParser = z.object({
  name: z.string(),
  manufacturer: z.string().optional(),
  price: z.number(),
  quantity: z.number().optional(),
  quality: z.number().optional(),
  description: z.string().optional(),
  status: z.enum(["free", "licenced", "illegal"]),
  concealment: z.number().optional(),
  stats_primary: z
    .object({
      hit: z.string().or(z.number()),
    })
    .optional(),
  stats_secondary: z.record(z.string(), z.number()).optional(),
  ranges: z
    .object({
      contact: z.object({
        label: z.string().optional(),
        base: z.number(),
      }),
      near: z.object({
        label: z.string().optional(),
        base: z.number(),
      }),
      short: z.object({
        label: z.string().optional(),
        base: z.number(),
      }),
      mid: z.object({
        label: z.string().optional(),
        base: z.number(),
      }),
      far: z.object({
        label: z.string().optional(),
        base: z.number(),
      }),
    })
    .optional(),
  actions: z.record(z.string(), actionParser).optional(),
  slots: asPseudoArray(slotParser).optional(),
});

const statsParser = z.object({
  con: z.number(),
  con_mod: z.number(),
  agi: z.number(),
  agi_mod: z.number(),
  rea: z.number(),
  rea_mod: z.number(),
  for: z.number(),
  for_mod: z.number(),
  vol: z.number(),
  vol_mod: z.number(),
  log: z.number(),
  log_mod: z.number(),
  int: z.number(),
  int_mod: z.number(),
  cha: z.number(),
  cha_mod: z.number(),
  ess: z.number(),
  edge: z.number(),
  resist_phy: asLeaf(z.number()),
  resist_ment: asLeaf(z.number()),
  def_phy: asLeaf(z.number()),
  def_ment: asLeaf(z.number()),
  init_dice: z.number(),
  init_base: asLeaf(z.number()),
  action_maj: z.number(),
  action_min: z.number(),
  hit_phy: z.number(),
  hit_stun: z.number(),
  hit_over: z.number(),
  heal: asLeaf(z.number()),

  resist_drain: asLeaf(z.number()),

  mag: z.number().optional(),
  initiation: z.number().optional(),

  res: z.number().optional(),
  submersion: z.number().optional(),
  firewall: z.number().optional(),
  traitement: z.number().optional(),
  corruption: z.number().optional(),
  attaque: z.number().optional(),
});

const characterParser = z.object({
  name: z.string(),
  tags: asPseudoArray(z.string()),
  knowledges: asPseudoArray(z.string()),
  identities: asPseudoArray(identityParser),
  stats: statsParser,
  skills: z.record(z.string(), skillParser),
  traits: z.record(z.string(), traitParser),
  powers: z.record(z.string(), actionParser).optional(),
  actions_common: z.record(z.string(), actionParser).optional(),
  companions: z.record(z.string(), companionParser).optional(),
});

export const parseCharacter: (data: unknown) => SRCharacter =
  characterParser.parse;
