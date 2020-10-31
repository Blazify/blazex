import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class VarAssignNode {
  public positionStart: Position;
  public positionEnd: Position;

  constructor(
    public name: Token,
    public value: Nodes,
    public type: TYPES,
    public reassignable: boolean
  ) {
    this.positionStart = name.positionStart!;
    this.positionEnd = value.positionEnd!;
  }

  public represent(): string {
    return `(${this.name.represent(), this.value.represent()})`;
  }
}
