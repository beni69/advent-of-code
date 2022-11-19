export const hello = () => "hi";

export type ParseFn = (input: string) => any;
export type SolveFn<T extends (...args: any) => any> = (
  input: ReturnType<T>,
  ...args: any
) => number;

export const getInput = async (year: number, day: number, cookie: string) => {
  const fname = `.cache/input-${year}-${day}.txt`;
  try {
    return await Deno.readTextFile(fname);
  } catch (_) {
    console.debug("fetching input file...");
    const txt = await fetch(
      `https://adventofcode.com/${year}/day/${day}/input`,
      {
        headers: {
          cookie: "session=" + cookie,
        },
      }
    ).then((r) => r.text());
    Deno.mkdir(".cache", { recursive: true }); // recursive to not fail if it already exists
    Deno.writeTextFile(fname, txt);
    return txt;
  }
};
