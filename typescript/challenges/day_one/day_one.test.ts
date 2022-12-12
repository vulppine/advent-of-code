import DayOne from "./day_one";
// const dayOne: DayOne = require("./day_one");

test('Tests base case of day one', () => {
    const calories = DayOne.parseCaloriesFromString(`1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`);

    const result = calories.getHighestCalories();

    expect(result).toBe(24000);
})

test('Tests day one, part one', async () => {
    const calories = await DayOne.parseCaloriesFromFile("../input/day_one.input");

    const result = calories.getHighestCalories();

    console.log(result);
})

test('Tests day one, part two', async () => {
    const calories = await DayOne.parseCaloriesFromFile("../input/day_one.input");

    const result = calories.getThreeHighestCalories();

    console.log(result);
})