import { Position } from "../../error/position.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";

export class IfNode {
  public positionStart: Position;
  public positionEnd: Position;

  constructor(
    public cases: [Nodes, Nodes][],
    public elseCase: Nodes | null,
    public type: TYPES,
  ) {
    this.positionStart = cases[0][0].positionStart;
    this.positionEnd = elseCase?.positionEnd ??
      cases[this.cases.length - 1][0].positionEnd;
  }

  public represent(): string {
    return `(IF ${
      this.cases.map((elif) => elif.map((sub) => sub.represent()).join(" -> "))
        .join(", ELSE IF ")
    }, ELSE ${this.elseCase?.represent()})`;
  }
}
