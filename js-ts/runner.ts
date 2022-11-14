import { config as dotenv } from "https://deno.land/x/dotenv@v3.2.0/mod.ts";
import { parse as parseArgs } from "https://deno.land/std@0.164.0/flags/mod.ts";
import { getInput } from "./lib.ts";

const args = parseArgs(Deno.args);
if (!args.y || !args.d) {
  console.error("-y (year) or -d (day) argument missing");
  Deno.exit(1);
}
const file = `./${args.y}/day${args.d}.ts`;
console.debug(args, file);

let input = args.i ?? (await getInput(args.y, args.d, dotenv().AOC_COOKIE));

const mod = await import(file);
console.debug(mod);
const fns = ["parse", "p1", "p2"];
for (const fn of fns) {
  if (!(mod[fn] instanceof Function)) {
    console.warn("skipping " + fn);
    continue;
  }

  console.debug("running " + fn);
  const res = mod[fn](input);
  console.log(res);
  fn === "parse" && (input = res);
}
