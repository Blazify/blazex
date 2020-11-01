import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";
import { Token } from "../token.ts";

export class FuncDefNode {
  public positionStart: Position;
  public positionEnd: Position;
  public type: TYPES;

  constructor(
    public bodyNode: Nodes,
    public varName?: Token,
    public argNameTokens?: Token[],
  ) {
    if (varName) {
      this.positionStart = varName.positionStart!;
    } else if (this.argNameTokens!.length > 0) {
      this.positionStart = this.argNameTokens![0].positionStart!;
    } else {
      this.positionStart = this.bodyNode.positionStart;
    }

    this.type = bodyNode.type;
    this.positionEnd = bodyNode.positionEnd;
  }

  public represent(): string {
    return `<FUNCTION ${this.varName?.value ?? "anonymous"} [${
      this.argNameTokens?.map((token) => token.represent()).join(", ")
    }]>`;
  }
}
