import { Position } from "../../error/position.ts";
import { TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class NumberNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES;
  constructor(public token: Token) {
    this.type = token.type;
    this.positionStart = this.token.positionStart!;
    this.positionEnd = this.token.positionEnd!;
  }

  public represent(): string {
    return `${this.token.represent()}`;
  }
}
