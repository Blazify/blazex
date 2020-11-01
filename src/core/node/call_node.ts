import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";

export class CallNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES;

  constructor(public nodeToCall: Nodes, public argNodes: Nodes[]) {
    this.type = nodeToCall.type;
    this.positionStart = nodeToCall.positionStart;
    if (this.argNodes!.length > 0) {
      this.positionEnd = this.argNodes![0].positionEnd;
    } else {
      this.positionEnd = nodeToCall.positionEnd;
    }
  }

  public represent(): string {
    return `<${this.nodeToCall.represent()}: [${
      this.argNodes.map((node) => node.represent()).join(", ")
    }]>`;
  }
}
