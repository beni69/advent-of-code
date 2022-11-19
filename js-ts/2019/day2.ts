import { ParseFn, SolveFn } from "../lib.ts";

export const parse: ParseFn = (inp: string) => inp.split(",").map(Number);

export const p1 = (inp: number[], noun = 12, verb = 2) => {
  const nums = [...inp];
  nums[1] = noun;
  nums[2] = verb;
  for (let i = 0; i < nums.length; i += 4) {
    switch (nums[i]) {
      case 99:
        return nums[0];

      case 1:
        nums[nums[i + 3]] = nums[nums[i + 1]] + nums[nums[i + 2]];
        break;

      case 2:
        nums[nums[i + 3]] = nums[nums[i + 1]] * nums[nums[i + 2]];
        break;
    }
  }
  return -1;
};

export const p2: SolveFn<typeof parse> = (nums: number[]) => {
  const sol = 19690720;
  for (let noun = 0; noun < 100; noun++)
    for (let verb = 0; verb < 100; verb++) {
      if (p1(nums, noun, verb) == sol) return 100 * noun + verb;
    }
  return -1;
};
