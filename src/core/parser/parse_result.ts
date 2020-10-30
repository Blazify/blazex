import { Err } from "../../error/err.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { NumberNode } from "../node/number_nodes.ts";
import { UnaryOpNode } from "../node/unary_op_node.ts";
import { VarAcessNode } from "../node/var_access_node.ts";
import { VarAssignNode } from "../node/var_assign_node.ts";

export class ParseResult {
  public error: Err | null = null;
  public node:
    | NumberNode
    | BinOpNode
    | UnaryOpNode
    | VarAssignNode
    | VarAcessNode
    | null = null;
  public advanceCount = 0;

  public registerAdvancement() {
    this.advanceCount += 1;
  }

  public register(
    res: ParseResult
    | NumberNode
    | BinOpNode
    | UnaryOpNode
    | VarAssignNode
    | VarAcessNode,
  ):
    | NumberNode
    | BinOpNode
    | UnaryOpNode
    | VarAssignNode
    | VarAcessNode {
    if(res instanceof ParseResult) {
      this.advanceCount += res.advanceCount ?? 1;
    if (res.error) this.error = res.error;
    return res.node!;
    }

    return res;
  }

  public success(
    node: NumberNode | BinOpNode | UnaryOpNode | VarAssignNode | VarAcessNode,
  ) {
    this.node = node;
    return this;
  }

  public failure(error: Err) {
    if(!this.error) this.error = error;
    if(this.advanceCount == 0) this.error = error;
    return this;
  }
}
