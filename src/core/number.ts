import { Err } from "../error/err.ts";
import { Position } from "../error/position.ts";
import { RuntimeError } from "../error/runtimeerr.ts";
import { Lexer } from "./lexer.ts";

export class Number {
  public positionStart!: Position | null;
  public positionEnd!: Position | null;
  constructor(public value: number) {
    this.setPosition();
  }

  public setPosition(
    start: Position | null = null,
    end: Position | null = null,
  ) {
    this.positionStart = start;
    this.positionEnd = end;
    return this;
  }

  public addTo(other: Number): { result: Number | null, error: Err | null } | undefined {
    if (other instanceof Number) {
      return { result: new Number(this.value + other.value), error: null };
    }
  }

  public subBy(other: Number): { result: Number | null, error: Err | null } | undefined {
    if (other instanceof Number) {
      return { result: new Number(this.value - other.value), error: null };
    }
  }

  public multiBy(other: Number): { result: Number | null, error: Err | null } | undefined {
    if (other instanceof Number) {
      return { result: new Number(this.value * other.value), error: null };
    }
  }

  public divBy(other: Number): { result: Number | null, error: Err | null } | undefined {
    if (other instanceof Number) {
      if(other.value == 0) return { result: null, error: new RuntimeError(other.positionStart!, other.positionEnd!, "Division by 0 isn't possible") }
      return { result: new Number(this.value / other.value), error: null };
    }
  }

  public represent() {
    const { tokens, errors } = new Lexer("Number", String(this.value))
      .makeTokens();
    if (errors) throw errors.forEach((e) => console.log(e.formatted()));
    return `${tokens![0].type}: ${tokens![0].value}`;
  }
}
