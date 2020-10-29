import { Position } from "../../error/position.ts";
import { Token } from "../token.ts";
import { BinOpNode } from "./binary_op_node.ts";
import { NumberNode } from "./number_nodes.ts";
import { UnaryOpNode } from "./unary_op_node.ts";

export class VarAssignNode {
    public positionStart: Position;
    public positionEnd: Position;

    constructor(public name: Token, public value: UnaryOpNode | NumberNode | BinOpNode) {
        this.positionStart = name.positionStart!;
        this.positionEnd = value.positionEnd!;  
    }
}