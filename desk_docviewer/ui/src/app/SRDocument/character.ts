import { ZodType, z } from "zod";

export type SRCharacter = z.infer<typeof characterParser>;

const valueParser = <T extends ZodType<any, any, any>>(v: T) =>
  z.object({
    value: v,
    metas: z.array(z.string()).optional(),
  });

const identityParser = z.object({
  name: valueParser(z.string().optional()),
  price: valueParser(z.number().optional()),
  quality: valueParser(z.number().optional()),
  contacts: z.array(
    z.object({
      name: z.string(),
      loyality: z.number(),
      connection: z.number(),
      description: z.string(),
    })
  ),
  lifestyles: z
    .array(
      z.object({
        name: z.string(),
        description: z.string().optional(),
      })
    )
    .optional(),
  licences: z.array(
    z.object({
      name: z.string(),
      description: z.string().optional(),
      price: z.number(),
      quality: z.number(),
    })
  ),
});

const characterParser = z.object({
  name: z.string(),
  tags: z.array(z.string()),
  knowledges: z.array(z.string()),
  identities: z.array(identityParser),
});

export const parseCharacter: (data: unknown) => SRCharacter =
  characterParser.parse;
