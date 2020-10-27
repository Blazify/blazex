import { Err } from "../error/err.ts";
import { IllegalCharecterError } from "../error/illegalchar.ts";
import { Position } from "../error/position.ts";
import {
  DIGITS,
  DIVIDE,
  FLOAT,
  INT,
  LEFT_PARENTHESIS,
  MINUS,
  MULTIPLY,
  PLUS,
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

  public makeTokens(): { tokens: Token[]; errors?: Err[] } {
    const tokens: Token[] = [];
    const errors: Err[] = [];

    while (this.currentCharecter !== null) {
      if ([" ", "\t"].includes(this.currentCharecter)) {
        this.advance();
      } else if (this.currentCharecter === "+") {
        tokens.push(new Token(PLUS));
        this.advance();
      } else if (this.currentCharecter === "-") {
        tokens.push(new Token(MINUS));
        this.advance();
      } else if (this.currentCharecter === "*") {
        tokens.push(new Token(MULTIPLY));
        this.advance();
      } else if (this.currentCharecter === "/") {
        tokens.push(new Token(DIVIDE));
        this.advance();
      } else if (this.currentCharecter === "(") {
        tokens.push(new Token(LEFT_PARENTHESIS));
        this.advance();
      } else if (this.currentCharecter === ")") {
        tokens.push(new Token(RIGHT_PARENTHESIS));
        this.advance();
      } else if (DIGITS.includes(Number(this.currentCharecter))) {
        tokens.push(this.makeNumber());
      } else {
        const positionStart = this.position.clone();
        errors.push(
          new IllegalCharecterError(
            positionStart,
            this.position,
            `Position ${this.position.index +
              1} at charecter ${this.currentCharecter}`,
          ),
        );
        this.advance();
      }
    }

    if (errors.length === 0) return { tokens };
    return { tokens, errors };
  }

  public makeNumber(): Token {
    let numberString = "";
    let dotCount = 0;

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

    if (dotCount === 0) return new Token(INT, Number(numberString));
    return new Token(FLOAT, Number(numberString));
  }
}
