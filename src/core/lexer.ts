import { Err } from "../error/err.ts";
import { ExcpectedChareterError } from "../error/excpectedchar.ts";
import { IllegalCharecterError } from "../error/illegalchar.ts";
import { Position } from "../error/position.ts";
import {
  ARROW,
  ASCII_LETTERS,
  ASCII_LETTERS_AND_DIGITS,
  CHAR,
  COLON,
  COMMA,
  DIGITS,
  DIVIDE,
  DOUBLE_EQUALS,
  EOF,
  EQUALS,
  FLOAT,
  GREATER_THAN,
  GREATER_THAN_EQUALS,
  IDENTIFIER,
  INT,
  KEYWORD,
  KEYWORDS,
  LEFT_PARENTHESIS,
  LESS_THAN,
  LESS_THAN_EQUALS,
  MINUS,
  MULTIPLY,
  NOT_EQUALS,
  PLUS,
  POWER,
  RIGHT_PARENTHESIS,
  STRING,
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
      } else if (this.currentCharecter === "^") {
        tokens.push(new Token(POWER, null, this.position));
        this.advance();
      } else if (this.currentCharecter === "!") {
        tokens.push(this.makeNotEquals());
      } else if (this.currentCharecter === "=") {
        tokens.push(this.makeEquals());
      } else if (this.currentCharecter === "<") {
        tokens.push(this.makeLessThan());
      } else if (this.currentCharecter === ">") {
        tokens.push(this.makeGreaterthan());
      } else if (this.currentCharecter === "&") {
        const { token, error } = this.makeAnd();
        if (error) {
          errors.push(error);
          return { errors };
        }
        tokens.push(token!);
      } else if (this.currentCharecter === "|") {
        const { token, error } = this.makeOr();
        if (error) {
          errors.push(error);
          return { errors };
        }
        tokens.push(token!);
      } else if (ASCII_LETTERS.includes(this.currentCharecter)) {
        tokens.push(this.makeIdentifier());
      } else if (DIGITS.includes(Number(this.currentCharecter))) {
        tokens.push(this.makeNumber());
      } else if (this.currentCharecter === ":") {
        tokens.push(new Token(COLON, null, this.position));
        this.advance();
      } else if (this.currentCharecter === ",") {
        tokens.push(new Token(COMMA, null, this.position));
        this.advance();
      } else if(this.currentCharecter === "'") {
        const { token, error } = this.makeChar();
        if(error) {
          errors.push(error);
          return { errors };
        } else if(token) {
          tokens.push(token);
        }
      } else if(this.currentCharecter === "\"") {
        tokens.push(this.makeString());
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

  public makeAnd(): { token?: Token; error?: Err } {
    const start = this.position.clone();
    this.advance();

    if (this.currentCharecter === "&") {
      this.advance();
      return { token: new Token(KEYWORD, "and", start, this.position) };
    }

    this.advance();
    return {
      error: new ExcpectedChareterError(
        start,
        this.position,
        "Expected '&' after '&'",
      ),
    };
  }

  public makeOr(): { token?: Token; error?: Err } {
    const start = this.position.clone();
    this.advance();
    if (this.currentCharecter === "|") {
      this.advance();
      return { token: new Token(KEYWORD, "or", start, this.position) };
    }
    this.advance();
    return {
      error: new ExcpectedChareterError(
        start,
        this.position,
        "Expected '|' after '|'",
      ),
    };
  }

  public makeNotEquals(): Token {
    const start = this.position.clone();
    this.advance();

    if (this.currentCharecter === "=") {
      this.advance();
      return new Token(NOT_EQUALS, null, start, this.position);
    }

    this.advance();
    return new Token(KEYWORD, "not", start, this.position);
  }

  public makeEquals(): Token {
    const start = this.position.clone();
    this.advance();

    if (this.currentCharecter === "=") {
      this.advance();
      return new Token(DOUBLE_EQUALS, null, start, this.position);
    } else if (this.currentCharecter === ">") {
      this.advance();
      return new Token(ARROW, null, start, this.position);
    }

    this.advance();
    return new Token(EQUALS, null, start, this.position);
  }

  public makeLessThan(): Token {
    const start = this.position.clone();
    this.advance();

    if (this.currentCharecter === "=") {
      this.advance();
      return new Token(LESS_THAN_EQUALS, null, start, this.position);
    }
    this.advance();
    return new Token(LESS_THAN, null, start, this.position);
  }

  public makeGreaterthan(): Token {
    const start = this.position.clone();
    this.advance();

    if (this.currentCharecter === "=") {
      this.advance();
      return new Token(GREATER_THAN_EQUALS, null, start, this.position);
    }
    this.advance();
    return new Token(GREATER_THAN, null, start, this.position);
  }

  public makeIdentifier(): Token {
    let identifier = "";
    const start = this.position.clone();
    while (
      this.currentCharecter !== null &&
      ASCII_LETTERS_AND_DIGITS.includes(this.currentCharecter)
    ) {
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

  public makeString(): Token {
    let string = "";
    const start = this.position.clone();
    let escape = false;
    this.advance();

    const escapeChars = {
      "n": "\n",
      "t": "\t"
    }

    while(this.currentCharecter !== null && this.currentCharecter !== "\"" || escape) {
      if(escape) {
        string += escapeChars[this.currentCharecter as keyof typeof escapeChars] ?? this.currentCharecter;
      } else {
        if(this.currentCharecter === "\\") {
        escape = true;
        } else {
        string += this.currentCharecter;
      }
    }
      this.advance();
      escape = false;
    }

    this.advance();

    return new Token(STRING, string, start, this.position)
  }

  public makeChar(): { token?: Token, error?: Err } {
    const start = this.position.clone();
    let char = '';

    this.advance();
    char += this.currentCharecter;

    this.advance();
    if(this.currentCharecter !== "'") {
      return { error: new ExcpectedChareterError(start, this.position, "Expected \"'\"") }
    }

    this.advance();
    return { token: new Token(CHAR, char, start, this.position) }
  }
}
