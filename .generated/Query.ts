// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { HSLink } from "./HSLink";

export type Query = { SanitizeRaw: { body: string, } } | { SanitizeTemplate: { body: string, extra_links: Array<HSLink>, } } | { BotLongDescription: { bot_id: string, } };