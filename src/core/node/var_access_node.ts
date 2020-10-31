import { Position } from "../../error/position.ts";
import { TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class VarAcessNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES;
  constructor(public token: Token) {
    this.type = token.value as TYPES;
    this.positionStart = token.positionStart!;
    this.positionEnd = token.positionEnd!;
  }

  public represent(): string {
    return `(${this.token.represent()})`;
  }
}
