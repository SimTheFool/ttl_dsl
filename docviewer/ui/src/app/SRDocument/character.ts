import { ZodType, z } from "zod";

export type SRCharacter = z.infer<typeof characterParser>;
export type SRIdentity = z.infer<typeof identityParser>;
export type SRTrait = z.infer<typeof traitParser>;
export type SRAction = z.infer<typeof actionParser>;
export type SRCompanion = z.infer<typeof companionParser>;
export type SRObject = z.infer<typeof objectParser>;

const asLeaf = <T extends ZodType<any, any, any>>(v: T) =>
  z.object({
    value: v,
    metas: asCoercedNullish(z.array(z.string())),
  });

const asPseudoArray = <T extends ZodType<any, any, any>>(v: T) =>
  z.record(z.string(), v);

const asCoercedNullish = <T extends ZodType<any, any, any>>(v: T) =>
  v.nullish().transform((v) => (v == null ? undefined : v));

const identityParser = z.object({
  name: z.string().nullish(),
  price: z.number().nullish(),
  nuyens: z.number().nullish(),
  quality: z.number().nullish(),
  contacts: asPseudoArray(
    z.object({
      name: z.string(),
      loyalty: z.number(),
      connection: z.number(),
      description: z.string(),
    })
  ).nullish(),
  lifestyle: z
    .object({
      name: z.string(),
      description: z.string().nullish(),
      price: z.number(),
    })
    .nullish(),
  licences: asPseudoArray(
    z.object({
      name: z.string(),
      description: z.string().nullish(),
      price: z.number(),
      quality: z.number(),
    })
  ).nullish(),
});

const skillParser = z.object({
  score: asCoercedNullish(z.number()),
  specializations: asPseudoArray(z.string()).nullish(),
  masterizations: asPseudoArray(z.string()).nullish(),
});

const traitParser = z.object({
  description: z.string(),
});

const actionParser = z.object({
  test: z.string().nullish(),
  major: z.number().nullish(),
  minor: z.number().nullish(),
  duration: z.string().nullish(),
  threshold: z.number().nullish(),
  maintained: z.boolean().nullish(),
  reaction: z.boolean().nullish(),
  range: z.enum(["perso", "contact", "LDV"]).nullish(),
  nature: z.enum(["physique", "mana", "duale"]).nullish(),
  zone: z.boolean().nullish(),
  description: z.string().nullable().nullish(),
  damage: z.number().nullish(),
  ammo: z.number().nullish(),
  gauge: z.number().nullish(),
  ammo_gauge: z.number().nullish(),
  ranges: z
    .object({
      contact: z.number(),
      near: z.number(),
      short: z.number(),
      mid: z.number(),
      far: z.number(),
    })
    .nullish(),
});

const companionParser = z.object({
  name: z.string().nullish(),
  dynamic: z.number().nullish(),
  stats_primary: z
    .object({
      major: z.number(),
      minor: z.number(),
      hit: z.number(),
    })
    .nullish(),
  stats_secondary: z.record(z.string(), z.number()).nullish(),
  skills: asPseudoArray(z.string()).nullish(),
  traits: asPseudoArray(traitParser).nullish(),
  actions: z.record(z.string(), actionParser).nullish(),
});

const slotParser = z.object({
  name: z.string().nullish(),
  size: z.enum(["S", "M", "L", "XL"]),
  concealment: asCoercedNullish(z.number()),
});

const objectParser = z.object({
  name: z.string(),
  manufacturer: z.string().nullable().nullish(),
  price: z.number(),
  price_unit: z.number(),
  quantity: z.number().nullish(),
  quality: z.number().nullish(),
  description: z.string().nullish(),
  status: z.enum(["free", "licenced", "illegal"]),
  concealment: asCoercedNullish(z.number()),
  stats_primary: z
    .object({
      hit: z.number().nullish(),
    })
    .nullish(),
  stats_secondary: z.record(z.string(), z.number()).nullish(),
  ranges: z
    .object({
      contact: z.object({
        label: asCoercedNullish(z.number()),
        base: z.number(),
      }),
      near: z.object({
        label: asCoercedNullish(z.number()),
        base: z.number(),
      }),
      short: z.object({
        label: asCoercedNullish(z.number()),
        base: z.number(),
      }),
      mid: z.object({
        label: asCoercedNullish(z.number()),
        base: z.number(),
      }),
      far: z.object({
        label: asCoercedNullish(z.number()),
        base: z.number(),
      }),
    })
    .nullish(),
  actions: z.record(z.string(), actionParser).nullish(),
  slots: asPseudoArray(slotParser).nullish(),
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

  mag: z.number().nullish(),
  initiation: z.number().nullish(),

  res: z.number().nullish(),
  submersion: z.number().nullish(),
  firewall: z.number().nullish(),
  traitement: z.number().nullish(),
  corruption: z.number().nullish(),
  attaque: z.number().nullish(),
});

const characterParser = z.object({
  name: z.string(),
  tags: asPseudoArray(z.string()),
  knowledges: asPseudoArray(z.string()),
  identities: asPseudoArray(identityParser),
  stats: statsParser,
  skills: z.record(z.string(), skillParser),
  traits: z.record(z.string(), traitParser),
  powers: z.record(z.string(), actionParser).nullish(),
  actions_common: z.record(z.string(), actionParser).nullish(),
  actions_magic: z.record(z.string(), actionParser).nullish(),
  companions: z.record(z.string(), companionParser).nullish(),
  small_inventory: z.record(z.string(), objectParser).nullish(),
  big_inventory: z.record(z.string(), objectParser).nullish(),
});

export const parseCharacter: (data: unknown) => SRCharacter =
  characterParser.parse;
