import * as DayTwo from "./day_two";
import DayOne from "../day_one/day_one";

test('Test day two example case', () => {
    const rounds = DayTwo.parseGameRoundsFromString(`A Y
B X
C Z`, new DayTwo.GameStrategyBothSelectParser());

    const result = DayTwo.getRoundTotal(rounds);

    expect(result).toBe(15);
})

test('Test day two, part one', async () => {
    const rounds = await DayTwo.parseGameRoundsFromFile("../input/day_two.input", new DayTwo.GameStrategyBothSelectParser());

    const result = DayTwo.getRoundTotal(rounds);

    console.log(result);
})

test('Test day two example case part two', () => {
    const rounds = DayTwo.parseGameRoundsFromString(`A Y
B X
C Z`, new DayTwo.GameStrategyRoundResultSelectParser());

    const result = DayTwo.getRoundTotal(rounds);

    expect(result).toBe(12);
})

test('Test day two, part two', async () => {
    const rounds = await DayTwo.parseGameRoundsFromFile("../input/day_two.input", new DayTwo.GameStrategyRoundResultSelectParser());

    const result = DayTwo.getRoundTotal(rounds);

    console.log(result);
})