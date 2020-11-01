import { Position } from "../../error/position.ts";
import { TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class StringNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES = "String";
  constructor(public token: Token) {
    this.positionStart = this.token.positionStart!;
    this.positionEnd = this.token.positionEnd!;
  }

  public represent(): string {
    return `${this.token.represent()}`;
  }
}
