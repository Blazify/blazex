import { Err } from "../../error/err.ts";
import { BinOpNode } from "../node/binary_op_node.ts";
import { NumberNode } from "../node/number_nodes.ts";

export class ParseResult {
    public error: Err | null = null;
    public node: NumberNode | BinOpNode | null = null;

    public register<T = ParseResult | NumberNode | BinOpNode>(res: T) {
        if(res instanceof ParseResult) {
            if(res.error) this.error = res.error;
            return res.node;
        }

        return res;
    }

    public success(node: NumberNode | BinOpNode) {
        this.node = node;
        return this;
    }

    public failure(error: Err) {
        this.error = error;
        return this;
    }
}