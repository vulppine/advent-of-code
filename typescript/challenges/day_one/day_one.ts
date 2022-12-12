// Day one: parse the list, add the values in each section, sort.

// java-esque, uses classes

import {open} from "fs/promises";

class DayOne {
    private calories: Array<number> = new Array<number>();

    public getThreeHighestCalories() : number {
        this.calories = this.calories.sort((a, b) => a - b);

        return this.calories.slice(-3).reduce((prev, cur, _, __) => {
            return prev + cur;
        });
    }

    public getHighestCalories(): number {
        this.calories = this.calories.sort((a, b) => a - b);

        return this.calories[this.calories.length - 1];
    }

    static parseCaloriesFromStrings(input: Iterable<string>): DayOne {
        const result = new DayOne();
        let current = 0;

        for (const chars of input) {
            if (chars.length == 0) {
                result.calories.push(current);
                current = 0;
            } else {
                current += parseInt(chars);
            }
        }

        return result;
    }

    static async parseCaloriesFromFile(path: string) : Promise<DayOne> {
        const strings = new Array<string>();
        const file = await open(path);

        for await (const line of file.readLines()) {
            strings.push(line);
        }

        return this.parseCaloriesFromStrings(strings);
    }

    static parseCaloriesFromString(input: string): DayOne {
        return this.parseCaloriesFromStrings(input.split("\n"));
    }
}

export default DayOne;