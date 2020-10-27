import { Err } from "../../error/err.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { NumberNode } from "../node/number_nodes.ts";
import { UnaryOpNode } from "../node/unary_op_node.ts";

export class ParseResult {
    public error: Err | null = null;
    public node: NumberNode | BinOpNode | UnaryOpNode | null = null;

    public register<T = ParseResult | NumberNode | BinOpNode>(res: T) {
        if(res instanceof ParseResult) {
            if(res.error) this.error = res.error;
            return res.node;
        }

        return res;
    }

    public success(node: NumberNode | BinOpNode | UnaryOpNode) {
        this.node = node;
        return this;
    }

    public failure(error: Err) {
        this.error = error;
        return this;
    }
}