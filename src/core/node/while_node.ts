import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";

export class WhileNode {
  public type: TYPES;
  public positionStart: Position;
  public positionEnd: Position;

  constructor(public conditionNode: Nodes, public bodyNode: Nodes) {
    this.type = bodyNode.type;
    this.positionStart = conditionNode.positionStart;
    this.positionEnd = bodyNode.positionEnd;
  }

  public represent(): string {
    return `(WHILE ${this.conditionNode.represent()} THEN ${this.bodyNode.represent()})`;
  }
}
