import { SolveFn } from "../lib.ts";

export const parse: SolveFn = (inp: string) => inp.split(",").map(Number);

export const p1 = (inp: number[]) => {
  const nums = inp;
  nums[1] = 12;
  nums[2] = 2;
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
};
