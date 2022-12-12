import {open} from "fs/promises";

enum GameSelection {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

function parseGameSelection(selection: string) : GameSelection {
    switch (selection) {
        case 'A':
        case 'X':
            return GameSelection.Rock;
        case 'B':
        case 'Y':
            return GameSelection.Paper;
        case 'C':
        case 'Z':
            return GameSelection.Scissors;
    }

    throw new Error("game selection could not be parsed");
}

// seriously, there HAS to be a way to calculate this: This isn't funny!
function getGameResult(opponent: GameSelection, ours: GameSelection) : GameResult {
    switch (opponent) {
        case GameSelection.Rock:
            switch (ours) {
                case GameSelection.Paper:
                    return GameResult.Win;
                case GameSelection.Scissors:
                    return GameResult.Loss;
            }

            break;
        case GameSelection.Paper:
            switch (ours) {
                case GameSelection.Rock:
                    return GameResult.Loss;
                case GameSelection.Scissors:
                    return GameResult.Win;
            }
            break;
        case GameSelection.Scissors:
            switch (ours) {
                case GameSelection.Rock:
                    return GameResult.Win;
                case GameSelection.Paper:
                    return GameResult.Loss;
            }

            break;
    }

    return GameResult.Draw;
}

enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6
}

function parseGameResult(input: string) : GameResult {
    switch (input) {
        case 'X':
            return GameResult.Loss;
        case 'Y':
            return GameResult.Draw;
        case 'Z':
            return GameResult.Win;
        default:
            throw new Error("incorrect letter passed into parseGameResult");
    }
}

function getGameSelectionFromResult(opponent: GameSelection, result: GameResult) : GameSelection {
    switch (result) {
        case GameResult.Loss:
            switch (opponent) {
                case GameSelection.Rock:
                    return GameSelection.Scissors;
                case GameSelection.Paper:
                    return GameSelection.Rock;
                case GameSelection.Scissors:
                    return GameSelection.Paper;
            }
            break;
        case GameResult.Win:
            switch (opponent) {
                case GameSelection.Rock:
                    return GameSelection.Paper;
                case GameSelection.Paper:
                    return GameSelection.Scissors;
                case GameSelection.Scissors:
                    return GameSelection.Rock;

            }
            break;
    }

    return opponent;
}

type GameRound = {
    opponentPlay: GameSelection;
    ourPlay: GameSelection;
};

interface IGameRoundParser {
    parseRound(opponent: string, ours: string) : GameRound;
}

export class GameStrategyBothSelectParser implements IGameRoundParser {
    parseRound(opponent: string, ours: string): GameRound {
        let opponentPlay = parseGameSelection(opponent);
        let ourPlay = parseGameSelection(ours);

        return { opponentPlay, ourPlay };
    }

}
export class GameStrategyRoundResultSelectParser implements IGameRoundParser {
    parseRound(opponent: string, ours: string): GameRound {
        const opponentPlay = parseGameSelection(opponent);
        const wantedResult = parseGameResult(ours);
        const ourPlay = getGameSelectionFromResult(opponentPlay, wantedResult);

        return { opponentPlay, ourPlay };
    }

}

function parseGameRound(round: string, parser: IGameRoundParser) : GameRound {
    if (round.length != 3) {
        throw new Error("incorrect length for round string");
    }

    const opponent = round.charAt(0);
    const ours = round.charAt(2);

    return parser.parseRound(opponent, ours);
}

function parseGameRounds(rounds: Iterable<String>, parser: IGameRoundParser) : Iterable<GameRound> {
    const result = new Array<GameRound>();
    for (const round of rounds) {
        if (round.length != 3) {
            throw new Error("incorrect length for round string");
        }

        const opponent = round.charAt(0);
        const ours = round.charAt(2);

        result.push(parser.parseRound(opponent, ours));
    }

    return result;
}

export function getRoundTotal(rounds: Iterable<GameRound>) : number {
    let result = 0;
    for (const round of rounds) {
        result += getGameResult(round.opponentPlay, round.ourPlay) + round.ourPlay;
    }

    return result;
}

export async function parseGameRoundsFromFile(path: string, parser: IGameRoundParser) : Promise<Iterable<GameRound>> {
    const file = await open(path);
    const result = new Array<GameRound>();

    for await (const line of file.readLines()) {
        result.push(parseGameRound(line, parser));
    }

    return result;
}

export function parseGameRoundsFromString(input: string, parser: IGameRoundParser) : Iterable<GameRound> {
    const split = input.split("\n");
    const result = new Array<GameRound>();

    for (const line of split) {
        result.push(parseGameRound(line, parser));
    }

    return result;
}