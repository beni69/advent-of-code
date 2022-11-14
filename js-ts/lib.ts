export const hello = () => "hi";

export type SolveFn = (input: string) => any;

export const getInput = async (year: number, day: number, cookie: string) => {
  const fname = `.cache/input-${year}-${day}.txt`;
  try {
    return await Deno.readTextFile(fname);
  } catch (_) {
    const txt = await fetch(
      `https://adventofcode.com/${year}/day/${day}/input`,
      {
        headers: {
          cookie: "session=" + cookie,
        },
      }
    ).then((r) => r.text());
    Deno.mkdir(".cache", { recursive: true });
    Deno.writeTextFile(fname, txt);
    return txt;
  }
};
