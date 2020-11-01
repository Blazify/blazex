import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class ForNode {
  public type: TYPES;
  public positionStart: Position;
  public positionEnd: Position;

  constructor(
    public varNameToken: Token,
    public startValue: Nodes,
    public endValue: Nodes,
    public bodyNode: Nodes,
    public stepValueNode?: Nodes,
  ) {
    this.type = bodyNode.type;
    this.positionStart = varNameToken.positionStart!;
    this.positionEnd = bodyNode.positionEnd;
  }

  public represent(): string {
    return `FOR ${this.varNameToken.represent()} ${this.startValue.represent()} ${this.endValue.represent()} ${
      this.stepValueNode?.represent()
    } ${this.bodyNode.represent()}`;
  }
}
