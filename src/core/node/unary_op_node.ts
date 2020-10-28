import { Position } from "../../error/position.ts";
import { ParseResult } from "../parser/parse_result.ts";
import { Token } from "../token.ts";
import { BinOpNode } from "./binary_op_node.ts";
import { NumberNode } from "./number_nodes.ts";

export class UnaryOpNode {
  public positionStart: Position;
  public positionEnd: Position;
  constructor(
    public opToken: Token,
    public node: BinOpNode | NumberNode | UnaryOpNode,
  ) {
    this.positionStart = this.node.positionStart;
    this.positionEnd = this.node.positionEnd;
  }

  public represent(): string {
    return `(${this.opToken.represent()}, ${this.node.represent()})`;
  }
}
