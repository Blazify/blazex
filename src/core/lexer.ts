import { Err } from "../error/err.ts";
import { IllegalCharecterError } from "../error/illegalchar.ts";
import { Position } from "../error/position.ts";
import {
  ASCII_LETTERS,
  ASCII_LETTERS_AND_DIGITS,
  DIGITS,
  DIVIDE,
  EOF,
  EQUALS,
  FLOAT,
  IDENTIFIER,
  INT,
  KEYWORD,
  KEYWORDS,
  LEFT_PARENTHESIS,
  MINUS,
  MULTIPLY,
  PLUS,
  POWER,
  RIGHT_PARENTHESIS,
} from "../utils/constants.ts";
import { Token } from "./token.ts";

export class Lexer {
  public position: Position;
  public currentCharecter: string | null = null;

  public constructor(fileName: string, public text: string) {
    this.position = new Position(-1, 0, -1, fileName, text);
    this.advance();
  }

  public advance(): void {
    this.position.advance(this.currentCharecter ?? "");
    this.currentCharecter = this.text.length > this.position.index
      ? this.text[this.position.index]
      : null;
  }

  public makeTokens(): { tokens?: Token[]; errors?: Err[] } {
    const tokens: Token[] = [];
    const errors: Err[] = [];

    while (this.currentCharecter !== null) {
      if ([" ", "\t"].includes(this.currentCharecter)) {
        this.advance();
      } else if (this.currentCharecter === "+") {
        tokens.push(new Token(PLUS, null, this.position));
        this.advance();
      } else if (this.currentCharecter === "-") {
        tokens.push(new Token(MINUS, null, this.position));
        this.advance();
      } else if (this.currentCharecter === "*") {
        tokens.push(new Token(MULTIPLY, null, this.position));
        this.advance();
      } else if (this.currentCharecter === "/") {
        tokens.push(new Token(DIVIDE, null, this.position));
        this.advance();
      } else if (this.currentCharecter === "(") {
        tokens.push(new Token(LEFT_PARENTHESIS, null, this.position));
        this.advance();
      } else if (this.currentCharecter === ")") {
        tokens.push(new Token(RIGHT_PARENTHESIS, null, this.position));
        this.advance();
      } else if(this.currentCharecter === "^") {
        tokens.push(new Token(POWER, null, this.position));
        this.advance();
      } else if(this.currentCharecter === "=") {
        tokens.push(new Token(EQUALS, null, this.position));
        this.advance();
      } else if(ASCII_LETTERS.includes(this.currentCharecter)) {
        tokens.push(this.makeIdentifier());
      } else if (DIGITS.includes(Number(this.currentCharecter))) {
        tokens.push(this.makeNumber());
      } else {
        const start = this.position.clone();
        const char = this.currentCharecter;
        this.advance();
        errors.push(
          new IllegalCharecterError(start, this.position, "'" + char + "'"),
        );
        return { errors };
      }
    }

    tokens.push(new Token(EOF, null, this.position));

    return { tokens };
  }

  public makeIdentifier(): Token {
    let identifier = "";
    const start = this.position.clone();
    while(this.currentCharecter !== null && ASCII_LETTERS_AND_DIGITS.includes(this.currentCharecter)) {
      identifier += this.currentCharecter;
      this.advance();
    }

    const type = KEYWORDS.includes(identifier) ? KEYWORD : IDENTIFIER;
    return new Token(type, identifier, start, this.position);
  }

  public makeNumber(): Token {
    let numberString = "";
    let dotCount = 0;
    const start = this.position.clone();

    while (
      this.currentCharecter !== null &&
      DIGITS.includes(
        this.currentCharecter === "."
          ? String(this.currentCharecter)
          : Number(this.currentCharecter),
      )
    ) {
      if (this.currentCharecter === ".") {
        if (dotCount === 1) break;
        dotCount += 1;
        numberString += ".";
        this.advance();
      } else {
        numberString += this.currentCharecter;
        this.advance();
      }
    }

    if (dotCount === 0) {
      return new Token(INT, Number(numberString), start, this.position);
    }
    return new Token(FLOAT, Number(numberString), start, this.position);
  }
}
