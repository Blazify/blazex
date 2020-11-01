import { Position } from "../../error/position.ts";
import { TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class CharNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES = "Char";
  constructor(public token: Token) {
    this.positionStart = this.token.positionStart!;
    this.positionEnd = this.token.positionEnd!;
  }

  public represent(): string {
    return `${this.token.represent()}`;
  }
}
