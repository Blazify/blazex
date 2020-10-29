import { Position } from "../error/position.ts";
import { TYPES } from "../utils/constants.ts";

export class Token {
  public positionStart: Position | null = null;
  public positionEnd: Position | null = null;
  public constructor(
    public type: TYPES,
    public value: unknown = null,
    positionStart: Position | null = null,
    positionEnd: Position | null = null,
  ) {
    if (positionStart) {
      this.positionStart = positionStart.clone();
      this.positionEnd = positionStart.clone();
      this.positionEnd.advance("");
    }

    if (positionEnd) {
      this.positionEnd = this.positionEnd;
    }
  }

  public match(type: TYPES, value: unknown): boolean {
    return this.type === type && this.value === value;
  }

  public represent(): string {
    if (this.value) return `${this.type}: ${this.value}`;
    return `${this.type}`;
  }
}
