import { config as dotenv } from "https://deno.land/x/dotenv@v3.2.0/mod.ts";
import { parse as parseArgs } from "https://deno.land/std@0.166.0/flags/mod.ts";
import { getInput } from "./lib.ts";

const args = parseArgs(Deno.args);
if (args.h || args.help)
  void console.log(
    `
Usage:
  -h | --help -> this message
  -v -> verbose mode
  --dl -> only download the input, don't solve
  -y: year
  -d: day
  -i: custom string to use as input
`.trim()
  ) || Deno.exit(0);

if (!args.y || !args.d) {
  console.error("-y (year) or -d (day) argument missing. run with -h for help");
  Deno.exit(1);
}
const file = `./${args.y}/day${args.d}.ts`;
args.v && console.debug(args, file);

let input = args.i ?? (await getInput(args.y, args.d, dotenv().AOC_COOKIE));

if (args.dl) Deno.exit(0);

const mod = await import(file);
args.v && console.debug(mod);
const fns = ["parse", "p1", "p2"];
for (const fn of fns) {
  if (!(mod[fn] instanceof Function)) {
    args.v && console.warn("skipping " + fn);
    continue;
  }

  args.v && console.debug("running " + fn);
  console.time(fn);
  const res = mod[fn](input);
  console.timeEnd(fn);

  if (fn === "parse") {
    input = res;
    args.v && console.debug("parse:", res);
  } else console.log(fn + ":", res);
}
