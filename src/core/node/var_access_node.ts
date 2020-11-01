import { Position } from "../../error/position.ts";
import { LangTypes, TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class VarAcessNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES;
  constructor(public token: Token) {
    this.type = LangTypes.includes(token.value as any)
      ? token.value as TYPES
      : "IDENTIFIER";
    this.positionStart = token.positionStart!;
    this.positionEnd = token.positionEnd!;
  }

  public represent(): string {
    return `(${this.token.represent()})`;
  }
}
