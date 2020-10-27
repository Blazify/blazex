import { TYPES } from "../utils/constants.ts";

export class Token {
  public constructor(public type: TYPES, public value: unknown = null) {
  }

  public represent(): string {
    if (this.value) return `${this.type}: ${this.value}`;
    return `${this.type}`;
  }
}
