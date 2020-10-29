import { Err } from "../error/err.ts";
import { BinOpNode } from "./node/binary_op_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { UnaryOpNode } from "./node/unary_op_node.ts";
import { Number } from "./number.ts";

export class RuntimeResult {
  public value: Number | null = null;
  public error: Err | null = null;

  public register(res: RuntimeResult | Number) {
    if (res instanceof RuntimeResult) {
      if (res.error) this.error = res.error;
      return res.value;
    }
    this.value = res;
    return this.value;
  }

  public success(value: Number) {
    this.value = value;
    return this;
  }

  public failure(err: Err) {
    this.error = err;
    return this;
  }
}
