import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class BinOpNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES;
  constructor(
    public leftNode: Nodes,
    public opToken: Token,
    public rightNode: Nodes,
  ) {
    this.type = this.leftNode.type && this.rightNode.type;
    this.positionStart = this.leftNode.positionStart;
    this.positionEnd = this.rightNode.positionEnd;
  }

  public represent(): string {
    return `(${this.leftNode.represent()}, ${this.opToken.represent()}, ${this.rightNode.represent()})`;
  }
}
