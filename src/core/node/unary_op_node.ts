import { ParseResult } from "../parser/parse_result.ts";
import { Token } from "../token.ts";
import { BinOpNode } from "./binary_op_node.ts";
import { NumberNode } from "./number_nodes.ts";

export class UnaryOpNode {
  constructor(public opToken: Token, public node: BinOpNode | NumberNode | UnaryOpNode | ParseResult) {}

  public represent(): string {
    return `(${this.opToken.represent()}, ${this.node instanceof ParseResult ? this.node.node?.represent() : this.node.represent()})`;
  }
}
