import { Token } from "../token.ts";

export class NumberNode {
  constructor(public token: Token) {}

  public represent(): string {
    return `${this.token.represent()}`;
  }
}
