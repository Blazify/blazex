import { Position } from "../../error/position.ts";
import { Token } from "../token.ts";

export class NumberNode {
  public positionStart: Position;
  public positionEnd: Position;
  constructor(public token: Token) {
    this.positionStart = this.token.positionStart!;
    this.positionEnd = this.token.positionEnd!;
  }

  public represent(): string {
    return `${this.token.represent()}`;
  }
}
