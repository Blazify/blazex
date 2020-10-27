import { Token } from "../token.ts";
import { NumberNode } from "./number_nodes.ts";

export class BinOpNode {
  constructor(
    public leftNode: NumberNode | BinOpNode,
    public opToken: Token,
    public rightNode: NumberNode | BinOpNode,
  ) {
  }

  public represent(): string {
    return `(${this.leftNode.represent()}, ${this.opToken.represent()}, ${this.rightNode.represent()})`;
  }
}
